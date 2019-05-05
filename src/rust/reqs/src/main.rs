/**
 * Champion recommendation entry-point
 * @author dmcfalls
 */

#[macro_use]
extern crate serde_derive;

mod champions;
mod matches;
mod winrates;
mod reqs;

use champions::load_champions;
use matches::load_matches;

fn main() {
  println!("Entering reqs");

  let champions = load_champions("/home/dmcfalls/dev/leeg/champions.json".to_string());
  println!("Loaded {} champions into vector with length {}", champions.count(), champions.len());

  let matches = load_matches("/home/dmcfalls/dev/leeg/matches.csv".to_string());
  println!("Loaded {} matches", matches.len());

  println!("Leaving reqs");
}
