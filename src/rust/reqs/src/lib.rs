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

const CHAMPIONS_FILE_PATH: &str = "/home/alex/leeg/champions.json";
const MATCHES_FILE_PATH: &str = "/home/alex/leeg/alex_last_500.csv";

pub fn handle_req_req(num_reqs: usize, team_picks: &Vec<String>, opp_picks: &Vec<String>, 
                        team_bans: &Vec<String>, opp_bans: &Vec<String>) -> Vec<String> {
    let champions = load_champions(CHAMPIONS_FILE_PATH.to_string());
    let matches = load_matches(MATCHES_FILE_PATH.to_string(), &champions);
    let req_service = SingleSummonerReqService::from_matches(&matches, &champions);
    req_service.req(&team_picks, &opp_picks, &team_bans, &opp_bans, num_reqs)
}
