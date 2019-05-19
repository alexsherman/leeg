/**
 * Common structs and traits for representing match data
 * @author dmcfalls
 * @author alexsherman
 */

extern crate csv;
extern crate serde_json;
extern crate chrono;
extern crate postgres;

use champions::Champion;
use champions::Champions;
use utils::postgres_utils::*;

use utils::bool_deserializer::bool_from_string;
use utils::vec_deserializer::vec_from_python_list;

use self::chrono::NaiveDateTime;
use self::postgres::Error;

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
    play_date: NaiveDateTime,
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

pub fn load_summoner_matches_from_db(summoner_name: String, champions: &Champions) -> Result<Vec<Match>, Error> {
    let conn = get_connection_to_matches_db()?;
    let mut id = String::from("");
    for row in &conn.query(Q_MOST_RECENT_ID_BY_NAME, &[&summoner_name]).unwrap() {
        id = row.get(0);
    }

    let mut matches: Vec<Match> = Vec::new();

    for row in &conn.query(Q_SUMM_MATCHES_FOR_ID, &[&id])? {
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
            play_date: row.get(9),
            game_version: row.get(10)
        };
        let game = Match::from_summoner_matches_table(db_match, &champions);
        matches.push(game);
    }
    Ok(matches) 
}

#[derive(Debug)]
pub struct GlobalMatch {
    same_wins: bool,
    same_team: Vec<usize>,
    opp_team: Vec<usize>
}

/*
 * Proof of concept method that, at least with our current DB size, we can efficiently get every
 * match of a given matchup. I've turned this into a vector of matches where the teams are 'same'
 * and 'opp' rather than blue and red.
 *
 * It should be pretty trivial to take this vector and calculate every champion present in those
 * games' scores and return.
 */
pub fn load_matches_with_champ_vecs(same_team: &Vec<String>, opp_team: &Vec<String>, champions: &Champions) -> Result<Vec<GlobalMatch>, Error> {
    let conn = get_connection_to_matches_db()?;
    let mut matches: Vec<GlobalMatch> = Vec::new();
    for row in &conn.query(Q_GLOBAL_MATCHES_BOTH_TEAM_BLUE, &[&same_team, &opp_team])? {
        let m = GlobalMatch {
            same_wins: row.get(0)),
            same_team: *champions.index_by_name(&row.get(1)),
            opp_team: *champions.index_by_name(&row.get(2))
        };
        matches.push(m);
    }
    for row in &conn.query(Q_GLOBAL_MATCHES_BOTH_TEAM_RED, &[&same_team, &opp_team])? {
        let opp_wins: bool = row.get(0);
        let m = GlobalMatch {
            same_wins: !opp_wins,
            same_team: *champions.index_by_name(&row.get(2)),
            opp_team: *champions.index_by_name(&row.get(1))
       };
        matches.push(m);
    }
    Ok(matches)
}
