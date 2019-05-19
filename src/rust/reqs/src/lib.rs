#[macro_use]
extern crate serde_derive;
#[macro_use]
mod matches;
mod scores;
mod utils;
mod champions;
mod reqs;
use champions::load_champions;
use matches::load_summoner_matches_from_db;
use reqs::SingleSummonerReqService;
use reqs::ReqService;

pub fn handle_req_req(summoner_name: &str, team_picks: &Vec<String>, opp_picks: &Vec<String>, 
                        team_bans: &Vec<String>, opp_bans: &Vec<String>) -> Vec<String> {
    let champions = load_champions(CHAMPIONS_FILE_PATH.to_string());
    let num_reqs = 10;
    let matches = load_summoner_matches_from_db(String::from(summoner_name), &champions).unwrap();
    let req_service = SingleSummonerReqService::from_matches(&matches, &champions);
    req_service.req(&team_picks, &opp_picks, &team_bans, &opp_bans, num_reqs)
}

pub fn handle_global_req_req(team_picks: &Vec<String>, opp_picks: &Vec<String>) -> Vec<String> {
    let champions = load_champions(CHAMPIONS_FILE_PATH.to_string());
    let matches = load_matches_with_champ_vecs(team_picks, opp_picks, champions.unwrap());
}
