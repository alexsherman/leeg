#[macro_use]
extern crate serde_derive;
extern crate itertools;
mod matches;
mod scores;
mod utils;
mod champions;
mod reqs;

use itertools::Itertools;
use champions::{load_champions,load_champions_with_role,Champions};
use matches::{load_summoner_matches_from_db, load_global_matches_from_db};
use reqs::{ReqService, SingleSummonerReqService, GlobalReqService, GlobalServiceWithWeight, combine_req_services};
use utils::redis_utils::{get_connection, Connection, get_cached_global_reqs, insert_cached_global_reqs};

const CHAMPIONS_FILE_PATH: &str = "/mnt/c/Users/Alex/Documents/dev/leeg/champions.json";
const ROLES_FILE_PATH: &str = "/mnt/c/Users/Alex/Documents/dev/leeg/champion_roles.json";
const SUFFICIENT_MATCH_THRESHOLD: usize = 1000;

pub fn handle_req_req(summoner_name: &str, team_picks: &Vec<String>, opp_picks: &Vec<String>, 
                        team_bans: &Vec<String>, opp_bans: &Vec<String>) -> Vec<String> {
    let champions = load_champions(CHAMPIONS_FILE_PATH.to_string());
    let num_reqs = 10;
    let matches = load_summoner_matches_from_db(String::from(summoner_name), &champions).unwrap();
    let req_service = SingleSummonerReqService::from_matches(&matches, &champions);
    req_service.req(&team_picks, &opp_picks, &team_bans, &opp_bans, num_reqs)
}

pub fn simple_handle_global_req_req(team_picks: &Vec<String>, opp_picks: &Vec<String>) -> Vec<String> {
    let champions = load_champions(CHAMPIONS_FILE_PATH.to_string());
    let matches = load_global_matches_from_db(&team_picks, &opp_picks, &champions).unwrap();
    let req_service = GlobalReqService::from_matches(&matches, &champions);
    req_service.req_banless(&team_picks, &opp_picks, 10)
}

pub fn handle_global_req_req(team_picks: &Vec<String>, opp_picks: &Vec<String>, roles: Option<Vec<String>>) 
                            -> Vec<String> {
    // connnect to redis
    let redis_connection = get_connection();
    // this will hold all the req structs which we will combine at the end
    let mut service_vec: Vec<GlobalServiceWithWeight> = Vec::new();
    let mut num_matches_analyzed: usize = 0;
    let champions = load_champions_with_role(CHAMPIONS_FILE_PATH.to_string(), ROLES_FILE_PATH.to_string());
    let weighted_service = get_or_create_global_req_service(&redis_connection, &team_picks, &opp_picks, &champions);
    num_matches_analyzed += weighted_service.weight;
    service_vec.push(weighted_service);
    // if we haven't analyzed enough matches, this is because the current query was too specific
    // to get enough data. so broaden the query by doing partial combinations of team_picks and opp_picks
    println!("{} matches analyzed so far", num_matches_analyzed);
    let mut team_n = team_picks.len();
    let mut opp_n = opp_picks.len();
    while num_matches_analyzed < SUFFICIENT_MATCH_THRESHOLD && (team_n > 1 || opp_n > 1) {
        team_n = match team_n {
            1 => 1,
            x => x - 1
        };
        opp_n = match opp_n {
            1 => 1,
            x => x - 1
        };
        for team_combination in team_picks.iter().cloned().combinations(team_n) {
            for opp_combination in opp_picks.iter().cloned().combinations(opp_n) {
                let w_service = get_or_create_global_req_service(&redis_connection, &team_combination, &opp_combination, &champions);
                num_matches_analyzed += w_service.weight;
                service_vec.push(w_service);
                println!("{} matches analyzed so far", num_matches_analyzed);
            }
        }
        println!("{} matches analyzed so far", num_matches_analyzed);
    }
   
    let combined_service = combine_req_services(&service_vec, roles);
    let res = combined_service.req_banless(&team_picks, &opp_picks, 20);    
    res
}    

/**
*   Attempts to get the requested global req service from cache. If not in cache, generate from database matches
*   and put it in the cache.
*/
fn get_or_create_global_req_service(conn: &Connection, team_picks: &Vec<String>, opp_picks: &Vec<String>, 
                                    champions: &Champions) 
                                    -> GlobalServiceWithWeight {
    let cached_entry = get_cached_global_reqs(&conn, &team_picks, &opp_picks);
    if cached_entry.is_ok() {
        return cached_entry.unwrap()
    }
    let matches = load_global_matches_from_db(&team_picks, &opp_picks, &champions).unwrap();
    let service = GlobalReqService::from_matches(&matches, &champions);
    // Currently trying to weight each service by the number of matches they have, but this is
    // very imperfect. But the idea is that if there are 2 matches for one combination and some
    // champion won in both of those, there needs to be some weight which prevents that 100% from
    // dominating the score
    let weighted_service = GlobalServiceWithWeight {
        req_service: service,
        weight: matches.len()
    };
    insert_cached_global_reqs(&conn, &team_picks, &opp_picks, weighted_service.clone());
    weighted_service
}
