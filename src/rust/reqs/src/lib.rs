#[macro_use]
extern crate serde_derive;
#[macro_use]
mod matches;
mod scores;
mod utils;
mod champions;
mod reqs;
use champions::load_champions;
use matches::load_matches;
use reqs::SingleSummonerReqService;
use reqs::ReqService;

const CHAMPIONS_FILE_PATH: &str = "/home/leeg/champions.json";
const MATCHES_FILE_PATH: &str = "/home/leeg/dans_last_100.csv";

pub fn handle_req_req(num_reqs: usize, team_picks: &Vec<String>, opp_picks: &Vec<String>) -> Vec<String> {
    let champions = load_champions(CHAMPIONS_FILE_PATH.to_string());
    let matches = load_matches(MATCHES_FILE_PATH.to_string(), &champions);
    let req_service = SingleSummonerReqService::from_matches(&matches, &champions);
    req_service.req(&team_picks, &opp_picks, num_reqs)
}
