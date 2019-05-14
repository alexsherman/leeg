/**
 * Common structs and traits for representing match data
 * @author dmcfalls
 */

extern crate csv;
extern crate serde_json;
extern crate chrono;
extern crate postgres;
extern crate toml;

use champions::Champion;
use champions::Champions;

use utils::bool_deserializer::bool_from_string;
use utils::vec_deserializer::vec_from_python_list;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const ALLIED_CHAMPS_SIZE: usize = 4;
const ENEMY_CHAMPS_SIZE: usize = 5;

/**
 * Config toml file to connect to database.
 */

#[derive(Deserialize)]
struct Config {
    database: String,
    host: String,
    user: String,
    password: String,
    port: String
}


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

/**
 * Representation of match data matching the columns in matches.summoner_matches table
 */

struct AllSummonersMatch {
    summoner_id: String,  //id
    summoner_name: String, //name
    summoner_win: bool, //wins
    champion: String, //champion
    same_team_champs: Vec<String>,
    opp_team_champs: Vec<String>,
    same_team_bans: Vec<String>,
    opp_team_bans: Vec<String>,
    match_id: i64,
  //  play_date: chrono::NaiveDateTime,
    game_version: String
}



impl Match {

	pub fn get_summoner_champion_idx(&self) -> usize {
		self.summoner_champion_idx
	}

	pub fn is_summoner_win(&self) -> bool {
		self.summoner_win
    }

	pub fn get_same_team_champion_idxs(&self) -> &Vec<usize> {
		&(self.same_team_champion_idxs)
	}

	pub fn get_opposing_team_champion_idxs(&self) -> &Vec<usize> {
		&(self.opposing_team_champion_idxs)
	}
 
	fn from_raw_match(raw_match: &RawMatch, champions: &Champions) -> Match {
	  let same_team_champs: Vec<String> = vec_from_python_list(&raw_match.same_team_champs);
	  let mut same_team_champion_idxs: Vec<usize> = Vec::with_capacity(ALLIED_CHAMPS_SIZE);
	  for champion_name in same_team_champs {
	  	same_team_champion_idxs.push(*champions.index_by_name(&champion_name));
	  }
	  let opposite_team_champs: Vec<String> = vec_from_python_list(&raw_match.opposite_team_champs);
	  let mut opposing_team_champion_idxs: Vec<usize> = Vec::with_capacity(ENEMY_CHAMPS_SIZE);
	  for champion_name in opposite_team_champs {
	  	opposing_team_champion_idxs.push(*champions.index_by_name(&champion_name));
	}


	  Match {
	  	summoner_champion_idx: champions.index_by_name(&raw_match.summoner_champ).clone(),
	  	summoner_win: bool_from_string(&raw_match.summoner_win),
	  	same_team_champion_idxs: same_team_champion_idxs,
	  	opposing_team_champion_idxs: opposing_team_champion_idxs
	  }
	}

    fn from_summoner_matches_table(db_match: AllSummonersMatch, champions: &Champions) -> Match {
        let mut same_team_champion_idxs: Vec<usize> = Vec::with_capacity(ALLIED_CHAMPS_SIZE);
        let mut opposing_team_champion_idxs: Vec<usize> = Vec::with_capacity(ENEMY_CHAMPS_SIZE);
        for champion_name in db_match.same_team_champs {
            same_team_champion_idxs.push(*champions.index_by_name(&champion_name));
        }
        for champion_name in db_match.opp_team_champs {
            opposing_team_champion_idxs.push(*champions.index_by_name(&champion_name));
        }

        Match {
            summoner_champion_idx: champions.index_by_name(&db_match.champion).clone(),
            summoner_win: db_match.summoner_win,
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
		matches.push(game);
	}

	return matches;
}

pub fn load_summoner_matches_from_db(summoner_name: String, champions: &Champions) -> Result<Vec<Match>, postgres::Error> {
    let mut config_file = File::open(&Path::new("Db_config.toml")).expect("No db config toml found!");
    let mut config_string = String::new();
    config_file.read_to_string(&mut config_string)?;
    let config: Config = toml::from_str(&config_string).unwrap();
    let connection_string = format!("postgres://{}:{}@{}:{}/{}", config.user, config.password, config.host, config.port, config.database);
    println!("{}", connection_string);
    let conn = postgres::Connection::connect(connection_string, postgres::TlsMode::None)?;
    let id_query_string = "SELECT id from summoner_matches where name = $1 LIMIT 1";
    let games_query_string = "SELECT * from summoner_matches where id = $1";
    let mut id = String::from("");
    for row in &conn.query(id_query_string, &[&summoner_name]).unwrap() {
        id = row.get(0);
    }

    let mut matches: Vec<Match> = Vec::new();

    for row in &conn.query(games_query_string, &[&id])? {
        let db_match = AllSummonersMatch {
            summoner_id: row.get(0),
            summoner_name: row.get(1),
            summoner_win: row.get(2),
            champion: row.get(3),
            same_team_champs: row.get(4),
            opp_team_champs: row.get(5),
            same_team_bans: row.get(6),
            opp_team_bans: row.get(7),
            match_id: row.get(8),
          //  play_date: row.get(9),
            game_version: row.get(10)
        };
        let game = Match::from_summoner_matches_table(db_match, &champions);
        matches.push(game);
    }
    Ok(matches) 
}
