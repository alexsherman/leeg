#[macro_use]
extern crate serde_derive;
extern crate itertools;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate simple_error;
extern crate crossbeam;
extern crate r2d2;
extern crate r2d2_postgres;
mod matches;
mod scores;
mod utils;
mod champions;
mod reqs;
mod summoner;

use itertools::Itertools;
pub use champions::{load_champions,load_champions_with_role,Champions, load_champions_from_db};
use matches::{load_global_matches_from_db, GlobalMatch, GlobalMatchMatrices};
use reqs::{GlobalReqService, NamedGlobalService, GlobalServiceWithWeight, combine_req_services};
use utils::redis_utils::{get_connection, Connection, get_cached_global_reqs, insert_cached_global_reqs, REDIS_DEFAULT_EXPIRE_TIME};
pub use utils::postgres_utils::{get_connection_to_matches_db, get_connection_string};
use summoner::*;
use utils::summoner_utils::Region;
use std::error::Error;
use crossbeam::crossbeam_channel::unbounded;
use crossbeam::thread;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

const CHAMPIONS_FILE_PATH: &str = "/mnt/c/Users/Alex/Documents/dev/leeg/champions.json";
const ROLES_FILE_PATH: &str = "/mnt/c/Users/Alex/Documents/dev/leeg/champion_roles.json";
const SUFFICIENT_MATCH_THRESHOLD: usize = 10000;


/*
    TODO add comments
*/
pub fn handle_global_req_req(team_picks: &Vec<String>, opp_picks: &Vec<String>, 
                             roles: Option<Vec<String>>, champions: &Champions,
                             pool: Pool<PostgresConnectionManager>) 
                            -> Result<Vec<String>, Box<Error>> {
    // connnect to redis
    let redis_connection = get_connection()?;
    // this will hold all the req structs which we will combine at the end
    let mut service_vec: Vec<GlobalServiceWithWeight> = Vec::new();
    let mut num_matches_analyzed: usize = 0;
    let weighted_service = get_or_create_global_req_service(&redis_connection, pool.clone(), 
                                                            &team_picks, 
                                                            &opp_picks, 
                                                            &champions, 
                                                            true);
    num_matches_analyzed += weighted_service.weight;
    service_vec.push(weighted_service);
    // if we haven't analyzed enough matches, this is because the current query was too specific
    // to get enough data. so broaden the query by doing partial combinations of team_picks and opp_picks
    println!("{} matches analyzed so far", num_matches_analyzed);
    let mut team_n = 2;
    let mut opp_n = 2;
    // Create an unbounded channel.
    let (s, r) = unbounded();
    let mut n_threads = 0;
    thread::scope(|scope| {
            while team_n >= 0 || opp_n >= 0 {
                for team_combination in team_picks.iter().cloned().combinations(team_n) {
                    for opp_combination in opp_picks.iter().cloned().combinations(opp_n) {
                        n_threads += 1;
                        let s2 = s.clone();
                        let tc2 = team_combination.clone();
                        let pool2 = pool.clone();
                        // use scoped threads until async syntax formalized
                        scope.spawn(move |_| {
                            let mut w_service = get_or_create_global_req_service(&get_connection().unwrap(),
                                                                                 pool2, 
                                                                                 &tc2, 
                                                                                 &opp_combination, 
                                                                                 &champions, 
                                                                                 false);
                            w_service.weight *= (team_n * opp_n) + 1;
                            println!("{}", w_service.weight);
                            s2.send(w_service).unwrap();      
                        });               
                    }
                }

                if team_n > opp_n && team_n > 0 {
                    team_n -= 1;
                } else if opp_n > 0 {
                    opp_n -= 1;
                } else {
                    break;
                }
            }
    }).unwrap();

    for _ in 0..n_threads {
        let thread_created_service = r.recv().unwrap();
        service_vec.push(thread_created_service);
    }
     
    let combined_service = combine_req_services(&service_vec, &team_picks, &opp_picks, roles, &champions);
    let res = combined_service.req_banless(&champions, 144);    
    Ok(res)
}    

/**
*   Attempts to get the requested global req service from cache. If not in cache, generate from database matches
*   and put it in the cache.
*/
fn get_or_create_global_req_service(conn: &Connection, pool: Pool<PostgresConnectionManager>, 
                                    team_picks: &Vec<String>, opp_picks: &Vec<String>, 
                                    champions: &Champions, derive: bool) 
                                    -> GlobalServiceWithWeight {
    let cached_entry = get_cached_global_reqs(&conn, &team_picks, &opp_picks);
    if cached_entry.is_ok() {
        return cached_entry.unwrap()
    }
    let matches = load_global_matches_from_db(&team_picks, &opp_picks, &champions, pool).unwrap();
    let service = GlobalReqService::from_matches(&matches, &team_picks, &opp_picks, champions.len());
    // Currently trying to weight each service by the number of matches they have, but this is
    // very imperfect. But the idea is that if there are 2 matches for one combination and some
    // champion won in both of those, there needs to be some weight which prevents that 100% from
    // dominating the score
    //todo - increase weight by specificity
    // 10 games of exact comp worth more than 10 games of less specific comp
    let weighted_service = GlobalServiceWithWeight {
        req_service: service,
        weight: matches.len()
    };
    insert_cached_global_reqs(&conn, &team_picks, &opp_picks, weighted_service.clone(), Some(REDIS_DEFAULT_EXPIRE_TIME));
    if derive {
        derive_and_cache_granular_services(&conn, &matches, &team_picks, &opp_picks, &champions);
    }
    
    weighted_service
}

/**
*    Given a list of matches and the team/opp picks which were used to query those matches, create a matrix of matches from those matches
*    where each inner Vec represents the subset of matches in which champions[i] was present (on team and opp respectively).
*    Then, create a service for that set of matches and cache it. 
*
*/
fn derive_and_cache_granular_services(conn: &Connection, matches: &Vec<GlobalMatch>, team_picks: &Vec<String>, 
                                      opp_picks: &Vec<String>, champions: &Champions) 
                                      {
    let derived_matrices = GlobalMatchMatrices::from_matches(&matches, &champions);
    create_then_cache_services(&conn, &derived_matrices, &team_picks, &opp_picks, &champions, true);
    create_then_cache_services(&conn, &derived_matrices, &team_picks, &opp_picks, &champions, false);
}

/** 
*   If potential_is_team is true, gets all of the games in same_derived_matrix, 
*   creates a service for each possible champion selection, and caches.
*   If potential_is_team is false, the we do the same with opp_derived_matrix
*/
fn create_then_cache_services(conn: &Connection, derived_matrices: &GlobalMatchMatrices, team_picks: &Vec<String>, 
                              opp_picks: &Vec<String>, champions: &Champions, potential_is_team: bool) {
    let matrix = match potential_is_team {
        true => &derived_matrices.same_derived_matrix,
        false => &derived_matrices.opp_derived_matrix
    };
    for (index, champ_match_vec) in matrix.iter().enumerate() {
        let champ_name = &champions.get_list()[index].name;
        if team_picks.contains(champ_name) {
            continue;
        }

        let service = GlobalReqService::from_matches(&champ_match_vec, &team_picks, &opp_picks, champions.len());
        let weighted_service = GlobalServiceWithWeight {
            req_service: service,
            weight: champ_match_vec.len()
        };

        match potential_is_team {
            true => {
                let mut potential_team_picks = team_picks.clone();
                potential_team_picks.push(champ_name.clone());
                insert_cached_global_reqs(&conn, &potential_team_picks, &opp_picks, weighted_service, Some(REDIS_DEFAULT_EXPIRE_TIME));    
            },
            false => {
                let mut potential_opp_picks = opp_picks.clone();
                potential_opp_picks.push(champ_name.clone());
                insert_cached_global_reqs(&conn, &team_picks, &potential_opp_picks, weighted_service, Some(REDIS_DEFAULT_EXPIRE_TIME));    
            }
        }
        
    }
}

/*
pub fn get_global_matrix() -> Vec<NamedGlobalService> {
    let champions = load_champions_with_role(CHAMPIONS_FILE_PATH.to_string(), ROLES_FILE_PATH.to_string());
    let redis_connection = get_connection();
    let mut service_vector: Vec<NamedGlobalService> = Vec::with_capacity(champions.get_list().len());
    for index in 0..champions.get_list().len() {
        let champ_name = &champions.get_list()[index].name;
        let empty_vec: Vec<String> = Vec::new();
        let mut single_champ_vec: Vec<String> = Vec::new();
        single_champ_vec.push(champ_name.clone());
        let r = get_or_create_global_req_service(&redis_connection, &empty_vec, &single_champ_vec, &champions, false);
        service_vector.push(NamedGlobalService {
            req_service: r.req_service,
            champ_name:  champ_name.clone()
        });
    }
    service_vector
}*/

pub fn get_summoner_mastery_by_name(name: String, pool: Pool<PostgresConnectionManager>) -> Result<Summoner, Box<Error>> {
    let region = Region::NA;
    let redis_connection = get_connection()?;
    Summoner::from_name_and_region(&redis_connection, pool, name, region)
}