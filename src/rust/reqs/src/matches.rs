/**
 * Common structs and traits for representing match data
 * @author dmcfalls
 * @author alexsherman
 */

extern crate csv;
extern crate serde_json;
extern crate postgres;

use champions::Champion;
use champions::Champions;
use utils::postgres_utils::*;

use utils::bool_deserializer::bool_from_string;
use utils::vec_deserializer::vec_from_python_list;

use self::postgres::Error;

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

#[derive(Debug, Clone)]
pub struct GlobalMatch {
    pub same_wins: bool,
    pub same_team: Vec<usize>,
    pub opp_team: Vec<usize>,
    pub same_bans: Vec<usize>,
    pub opp_bans: Vec<usize>
}

impl GlobalMatch {
	pub fn get_same_team_champion_idxs(&self) -> &Vec<usize> {
		&(self.same_team)
	}

	pub fn get_opp_team_champion_idxs(&self) -> &Vec<usize> {
		&(self.opp_team)
	}

    pub fn get_same_bans(&self) -> &Vec<usize> {
        &(self.same_bans)
    }

    pub fn get_opp_bans(&self) -> &Vec<usize> {
        &(self.opp_bans)
    }
}

/*
 * Proof of concept method that, at least with our current DB size, we can efficiently get every
 * match of a given matchup. I've turned this into a vector of matches where the teams are 'same'
 * and 'opp' rather than blue and red.
 *
 * It should be pretty trivial to take this vector and calculate every champion present in those
 * games' scores and return.
 */
pub fn load_global_matches_from_db(same_team: &Vec<String>, opp_team: &Vec<String>, champions: &Champions) -> Result<Vec<GlobalMatch>, Error> {
    println!("{:?}, {:?}", same_team, opp_team);
    if same_team.is_empty() && opp_team.is_empty() {
        return load_all_matches(&champions);
    }
    let conn = get_connection_to_matches_db()?;
    let mut matches: Vec<GlobalMatch> = Vec::new();
    for row in &conn.query(Q_GLOBAL_MATCHES_BOTH_TEAM_BLUE, &[&same_team, &opp_team])? {
        let same_champ_names = row.get(1);
        let opp_champ_names = row.get(2);
        let same_bans = row.get(3);
        let opp_bans = row.get(4);
        let m = GlobalMatch {
            same_wins: row.get(0),
            same_team: champions.idxs_from_names(&same_champ_names),
            opp_team: champions.idxs_from_names(&opp_champ_names),
            same_bans: champions.idxs_from_names(&same_bans),
            opp_bans: champions.idxs_from_names(&opp_bans)
        };
        matches.push(m);
    }
    for row in &conn.query(Q_GLOBAL_MATCHES_BOTH_TEAM_RED, &[&same_team, &opp_team])? {
        let opp_wins: bool = row.get(0);
        let same_champ_names = row.get(2);
        let opp_champ_names = row.get(1);
        let same_bans = row.get(4);
        let opp_bans = row.get(3);
        let m = GlobalMatch {
            same_wins: !opp_wins,
            same_team: champions.idxs_from_names(&same_champ_names),
            opp_team: champions.idxs_from_names(&opp_champ_names),
            same_bans: champions.idxs_from_names(&same_bans),
            opp_bans: champions.idxs_from_names(&opp_bans)
        };
        matches.push(m);
    }
    Ok(matches)
}

/**
*   Loads all matches, no champ restrictions. I think this is more efficient than passing
*   empty arrays, not sure if postgres optimizes it away. Also requires just 1 query
*   that we can just duplicate for same and opp.
*/
fn load_all_matches(champions: &Champions) -> Result<Vec<GlobalMatch>, Error> {
    let conn = get_connection_to_matches_db()?;
    let mut matches: Vec<GlobalMatch> = Vec::new();
    for row in &conn.query(Q_ALL_MATCHES, &[])? {
        let blue_wins: bool = row.get(0);
        let blue_champ_names = row.get(1);
        let red_champ_names = row.get(2);
        let blue_bans = row.get(3);
        let red_bans = row.get(4);
        let m = GlobalMatch {
            same_wins: blue_wins,
            same_team: champions.idxs_from_names(&blue_champ_names),
            opp_team: champions.idxs_from_names(&red_champ_names),
            same_bans: champions.idxs_from_names(&blue_bans),
            opp_bans: champions.idxs_from_names(&red_bans)
        };
        matches.push(m);
        let m_flipped = GlobalMatch {
            same_wins: !blue_wins,
            same_team: champions.idxs_from_names(&red_champ_names),
            opp_team: champions.idxs_from_names(&blue_champ_names),
            same_bans: champions.idxs_from_names(&red_bans),
            opp_bans: champions.idxs_from_names(&blue_bans)
        };
        matches.push(m_flipped);
    }
    Ok(matches)
}

pub struct GlobalMatchMatrices {
    pub same_derived_matrix: Vec<Vec<GlobalMatch>>,
    pub opp_derived_matrix: Vec<Vec<GlobalMatch>>
}

impl GlobalMatchMatrices {
    pub fn from_matches(matches: &Vec<GlobalMatch>, champions: &Champions) -> GlobalMatchMatrices {
        let mut same_derived_matrix: Vec<Vec<GlobalMatch>> = Vec::new();
        let mut opp_derived_matrix: Vec<Vec<GlobalMatch>> = Vec::new();
        for _ in 0..champions.get_list().len() {
            same_derived_matrix.push(Vec::new());
            opp_derived_matrix.push(Vec::new());
        }

        for m in matches {
            for c in &m.same_team {
                same_derived_matrix[*c].push((*m).clone());
            }
            for c in &m.opp_team {
                opp_derived_matrix[*c].push((*m).clone());
            }
        }
        GlobalMatchMatrices {
            same_derived_matrix: same_derived_matrix,
            opp_derived_matrix: opp_derived_matrix
        }
    }
}