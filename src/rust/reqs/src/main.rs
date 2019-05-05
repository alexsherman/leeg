/**
 * Champion recommendation entry-point
 * @author dmcfalls
 */

mod reqs;
mod matches;
mod winrates;
mod champions;

use champions::load_champions;

fn main() {
  println!("Entering reqs");

  let champions = load_champions("/home/dmcfalls/dev/leeg/champions.json".to_string());
  println!("{:?}", champions);

  println!("Leaving reqs");
}
