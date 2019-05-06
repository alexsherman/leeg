/**
 * Common structs and traits for representing match data
 * @author dmcfalls
 */

extern crate csv;
extern crate serde_json;

use champions::Champion;
use champions::Champions;

use utils::bool_deserializer::bool_from_string;
use utils::vec_deserializer::vec_from_python_list;

const ALLIED_CHAMPS_SIZE: usize = 4;
const ENEMY_CHAMPS_SIZE: usize = 5;

 /**
  * Details for a single game of league of legends, formatted to contain minimized details from
  * a single player's perspective, and with the assumption that the player is known
  */
#[derive(Debug)]
pub struct Match {
	summoner_champion_idx: usize,
	summoner_win: bool,
	same_team_champion_idxs: Vec<usize>,
	opposing_team_champion_idxs: Vec<usize>
}

/**
 * Serialized representation on match data in the format produced by summoner_history.py
 */
#[derive(Debug, Deserialize)]
struct RawMatch {
	summoner_name: String,
	summoner_champ: String,
	summoner_win: String,
	same_team_champs: String,
	opposite_team_champs: String,
	account_id: String,
	match_id: u64,
	game_version: String
}

impl Match {
 
	fn from_raw_match(raw_match: &RawMatch, champions: &Champions) -> Match {
	  let same_team_champs: Vec<String> = vec_from_python_list(&raw_match.same_team_champs);
	  let mut same_team_champion_idxs: Vec<usize> = Vec::with_capacity(ALLIED_CHAMPS_SIZE);
	  for champion_name in same_team_champs {
	  	same_team_champion_idxs.push(champions.index_by_name(&champion_name).unwrap());
	  }
	  let opposite_team_champs: Vec<String> = vec_from_python_list(&raw_match.opposite_team_champs);
	  let mut opposing_team_champion_idxs: Vec<usize> = Vec::with_capacity(ENEMY_CHAMPS_SIZE);
	  for champion_name in opposite_team_champs {
	  	opposing_team_champion_idxs.push(champions.index_by_name(&champion_name).unwrap());
	  }

	  Match {
	  	summoner_champion_idx: champions.index_by_name(&raw_match.summoner_champ).unwrap(),
	  	summoner_win: bool_from_string(&raw_match.summoner_win),
	  	same_team_champion_idxs: same_team_champion_idxs,
	  	opposing_team_champion_idxs: opposing_team_champion_idxs
	  }
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

pub fn load_matches(filename: String, champions: &Champions) -> Vec<Match> {
	let mut csv_reader = csv::Reader::from_path(filename).expect("Unable to read file");
	let mut matches: Vec<Match> = Vec::new();

	for result in csv_reader.deserialize() {
		let raw_match: RawMatch = result.unwrap();
		let game = Match::from_raw_match(&raw_match, &champions);
		println!("Adding a new match with match_id {}", raw_match.match_id);
		matches.push(game);
	}

	return matches;
}