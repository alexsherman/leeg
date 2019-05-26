#[macro_use]
extern crate serde_derive;
#[macro_use] extern crate itertools;
mod matches;
mod scores;
mod utils;
mod champions;
mod reqs;

use itertools::Itertools;
use champions::{load_champions,load_champions_with_role};
use matches::{load_summoner_matches_from_db, load_global_matches_from_db, GlobalMatch};
use reqs::{ReqService, SingleSummonerReqService, GlobalReqService, GlobalServiceWithWeight, combine_req_services};

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

pub fn handle_global_req_req(team_picks: &Vec<String>, opp_picks: &Vec<String>, roles: Option<Vec<String>>) -> Vec<String> {
    let redis_connection = utils::redis_utils::get_connection();

    // this will hold all the structs that we combine at the end
    let mut service_vec: Vec<GlobalServiceWithWeight> = Vec::new();
    let mut num_matches_analyzed: usize = 0;

    let champions = load_champions_with_role(CHAMPIONS_FILE_PATH.to_string(), ROLES_FILE_PATH.to_string());
    let matches = load_global_matches_from_db(&team_picks, &opp_picks, &champions).unwrap();
    num_matches_analyzed += matches.len();
    let req_service = GlobalReqService::from_matches(&matches, &champions);
    let w_service = GlobalServiceWithWeight {
        req_service: req_service,
        weight: matches.len()
    };
    //TEST INSERT
    let insert = utils::redis_utils::insert_cached_global_reqs(&redis_connection, &team_picks, &opp_picks, w_service.clone());
    service_vec.push(w_service);
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
                let m = load_global_matches_from_db(&team_combination, &opp_combination, &champions).unwrap();
                num_matches_analyzed += m.len();
                let req_service2 = GlobalReqService::from_matches(&m, &champions);
                // Currently trying to weight each service by the number of matches they have, but this is
                // very imperfect. But the idea is that if there are 2 matches for one combination and some
                // champion won in both of those, there needs to be some weight which prevents that 100% from
                // dominating the score
                let service = GlobalServiceWithWeight {
                    req_service: req_service2,
                    weight: m.len()
                };
                service_vec.push(service);
                println!("{} matches analyzed so far", num_matches_analyzed);
            }
        }
        println!("{} matches analyzed so far", num_matches_analyzed);
    }
   
    let combined_service = combine_req_services(&service_vec, roles);
    let res = combined_service.req_banless(&team_picks, &opp_picks, 20);    
    res
}    

