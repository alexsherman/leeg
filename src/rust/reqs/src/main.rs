/**
 * Champion recommendation entry-point
 * @author dmcfalls
 */

#[macro_use]
extern crate serde_derive;

mod champions;
mod matches;
mod scores;
mod reqs;
mod utils;

use champions::load_champions;
use matches::load_matches;
use reqs::SingleSummonerReqService;

fn main() {
  println!("Entering reqs");

  let champions = load_champions("/home/dmcfalls/dev/leeg/champions.json".to_string());
  println!("Loaded {} champions into vector with length {}", champions.count(), champions.len());

  let matches = load_matches("/home/dmcfalls/dev/leeg/dans_last_100.csv".to_string(), &champions);
  println!("Loaded {} matches", matches.len());

  let req_service = SingleSummonerReqService::from_matches(matches, &champions);
  // TODO: implement req_service and add example code here demonstrating its usage

  println!("Leaving reqs");
}
