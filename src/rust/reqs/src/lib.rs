#[macro_use]
extern crate serde_derive;
extern crate itertools;

#[macro_use]
extern crate log;
extern crate env_logger;
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

pub use utils::postgres_utils::{get_connection_string, FromDatabase};
pub use champions::{Champions, load_champions_from_db};
use matches::{GlobalMatch, GlobalMatchContainer, GlobalMatchMatrices};
use reqs::{GlobalReqService, GlobalServiceWithWeight, combine_req_services};
use utils::redis_utils::{get_connection, RedisConnection, Cacheable};
use itertools::Itertools;
use summoner::*;
use utils::summoner_utils::Region;
use std::error::Error;
use crossbeam::crossbeam_channel::unbounded;
use crossbeam::thread;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

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
    let weighted_service = get_or_create_global_req_service(&redis_connection, pool.clone(), 
                                                            &team_picks, 
                                                            &opp_picks, 
                                                            &champions, 
                                                            true)?;
    service_vec.push(weighted_service);
    let mut team_n: usize = 3;
    let mut opp_n: usize = 3;
    let (s, r) = unbounded();
    let mut n_threads = 0;
    thread::scope(|scope| {
        while team_n > 0usize || opp_n > 0usize {
            for team_combination in team_picks.iter().cloned().combinations(team_n) {
                for opp_combination in opp_picks.iter().cloned().combinations(opp_n) {
                    n_threads += 1;
                    let s2 = s.clone();
                    let tc2 = team_combination.clone();
                    let pool2 = pool.clone();
                    // async would be better but the ecosystem in to much flux rn for me to bother
                    scope.spawn(move |_| {
                        let mut w_service = get_or_create_global_req_service(&get_connection().unwrap(),
                                                                             pool2, 
                                                                             &tc2, 
                                                                             &opp_combination, 
                                                                             &champions, 
                                                                             false).unwrap();
                        w_service.weight *= (team_n * opp_n) + 1;
                        s2.send(w_service).unwrap();      
                    });               
                }
            }

            if team_n > opp_n {
                team_n -= 1;
            } else {
                opp_n -= 1;
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
fn get_or_create_global_req_service(conn: &RedisConnection, pool: Pool<PostgresConnectionManager>, 
                                    team_picks: &Vec<String>, opp_picks: &Vec<String>, 
                                    champions: &Champions, derive: bool) 
                                    -> Result<GlobalServiceWithWeight, Box<Error>> {
    match GlobalServiceWithWeight::with_picks(team_picks, opp_picks).from_cache(&conn) {
        Ok(cache_entry) => return Ok(cache_entry),
        Err(_) => ()
    };
    let match_container = GlobalMatchContainer::with_teams_and_champs(&team_picks, &opp_picks, &champions).from_database(pool)?;
    let weight = match_container.matches.len();
    let service = GlobalReqService::from_matches_container(&match_container);    
    let weighted_service = GlobalServiceWithWeight {
        req_service: service,
        weight: weight
    };
    weighted_service.insert_into_cache(&conn);
    if derive {
        derive_and_cache_granular_services(&conn, &match_container.matches, &team_picks, &opp_picks, &champions);
    }
    
    Ok(weighted_service)
}

/**
*    Given a list of matches and the team/opp picks which were used to query those matches, create a matrix of matches from those matches
*    where each inner Vec represents the subset of matches in which champions[i] was present (on team and opp respectively).
*    Then, create a service for that set of matches and cache it. 
*
*/
fn derive_and_cache_granular_services(conn: &RedisConnection, matches: &Vec<GlobalMatch>, team_picks: &Vec<String>, 
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
fn create_then_cache_services(conn: &RedisConnection, derived_matrices: &GlobalMatchMatrices, team_picks: &Vec<String>, 
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
        let mut weighted_service = GlobalServiceWithWeight {
            req_service: service,
            weight: champ_match_vec.len()
        };

        match potential_is_team {
            true => {
                weighted_service.req_service.team_picks.push(champ_name.clone());
                weighted_service.insert_into_cache(&conn);
            },
            false => {
                weighted_service.req_service.opp_picks.push(champ_name.clone());
                weighted_service.insert_into_cache(&conn);
            }
        }
        
    }
}

pub fn get_summoner_mastery_by_name(name: String, pool: Pool<PostgresConnectionManager>) -> Result<Summoner, Box<Error>> {
    let region = Region::NA;
    let redis_connection = get_connection()?;
    Summoner::from_name_and_region(&redis_connection, pool, name, region)
}