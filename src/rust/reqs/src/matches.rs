/**
 * Common structs and traits for representing match data
 * @author dmcfalls
 */

extern crate csv;
extern crate serde_json;

use champions::Champion;

 /**
  * Details for a single game of league of legends, formatted
  */
#[derive(Debug, Deserialize)]
pub struct Match {
	summoner_name: String,
	summoner_champ: Champion,
	summoner_win: bool,
	same_team_champions: Vec<Champion>,
	opposing_team_champions: Vec<Champion>
}

/**
 * Serialized representation on match data produced by scripts
 */
struct RawMatch {
	summoner_name: String,
	summoner_champ: String,
	summoner_win: bool,
	same_team_champs: Vec<String>,
	opposite_team_champs: Vec<String>
	account_id: String,
	match_id: u64,
	game_version: String
}

impl Match {

	fn from_raw_match(raw_match: RawMatch) -> Match {

	}

}

/**
 * Details for a single summoner within a single game of league of legends
 */
#[derive(Debug, Deserialize)]
pub struct Summoner {
	champion: Champion,
	role: Role,
	team: TeamColor,
	summoner_name: String,
	account_id: String
}

/**
 * Signifies either red team or blue team
 */
#[derive(Debug, Deserialize)]
pub enum TeamColor {
	Red,
	Blue
}

/**
 * Signifies a role on summoner's rift
 */
#[derive(Debug, Deserialize)]
pub enum Role {
	Top,
	Jungle,
	Middle,
	DuoCarry,
	DuoSupport,
	Unknown
}

pub fn load_matches(filename: String) -> Vec<Match> {
	let mut csv_reader = csv::Reader::from_path(filename).expect("Unable to read file");
	let mut matches: Vec<Match> = Vec::new();

	for result in csv_reader.deserialize() {
		let raw_match: RawMatch = result.unwrap();
		let game = Match::from_raw_match(raw_match);
		println!("{:?}", game);
		break;
	}

	return matches;
}