/**
 * Champion recommendation entry-point
 * @author dmcfalls
 */

#[macro_use]
extern crate serde_derive;

#[macro_use]
mod utils;

mod champions;
mod matches;
mod scores;
mod reqs;

use champions::load_champions;
use matches::load_matches;
use reqs::ReqService;
use reqs::SingleSummonerReqService;

use utils::string::into_string_vec;

// Change these to point to the desired file locations
const CHAMPIONS_FILE_PATH: &str = "/home/dmcfalls/dev/leeg/champions.json";
const MATCHES_FILE_PATH: &str = "/home/dmcfalls/dev/leeg/dans_last_100.csv";

// Change these to contain the desired partial team picks
const MY_TEAM_CHAMPS: &[&str] = &["Annie", "Nidalee", "Thresh", "Dr. Mundo"];
const OTHER_TEAM_CHAMPS: &[&str] = &["Kayn", "Yasuo", "Riven", "Jhin", "Nami"];

// Change these to contain the desired bans for each team
const MY_TEAM_BANS: &[&str] = &["Blitzcrank"];
const OTHER_TEAM_BANS: &[&str] = &["Ezreal", "Fizz"];

// Change this to the desired number of reqs
const N_REQS: usize = 3;

fn main() {
  println!("Entering reqs");

  let champions = load_champions(CHAMPIONS_FILE_PATH.to_string());
  println!("Loaded {} champions", champions.len());

  let matches = load_matches(MATCHES_FILE_PATH.to_string(), &champions);
  println!("Loaded {} matches", matches.len());

  let req_service = SingleSummonerReqService::from_matches(&matches, &champions);
  println!("Created champion recommendation service");

  let team_bans: Vec<String> = into_string_vec(MY_TEAM_BANS);
  let other_bans: Vec<String> = into_string_vec(OTHER_TEAM_BANS);

  println!("My team banned these champions: {:?}", team_bans);
  println!("Other team banned these champions: {:?}", other_bans);

  let my_team_champs: Vec<String> = into_string_vec(MY_TEAM_CHAMPS);
	let other_team_champs: Vec<String> = into_string_vec(OTHER_TEAM_CHAMPS);

  println!("My team has these champions: {:?}", my_team_champs);
  println!("Other team has these champions: {:?}", other_team_champs);

  let reqs = req_service.req(&my_team_champs, &other_team_champs, &team_bans, &other_bans, N_REQS);
  println!("Req service says: maybe I should play one of *these* champions: {:?}", reqs);

  println!("Leaving reqs");
}
