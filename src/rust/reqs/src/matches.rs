/**
 * Common structs and traits for representing match data
 * @author dmcfalls
 * @author alexsherman
 */
extern crate csv;
extern crate serde_json;
extern crate postgres;

use champions::Champions;
use utils::postgres_utils::*;

use self::postgres::Error;

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
pub fn load_global_matches_from_db(same_team: &Vec<String>, opp_team: &Vec<String>, 
                                   champions: &Champions, pool: ConnectionPool) 
                                   -> Result<Vec<GlobalMatch>, Error> {
    println!("{:?}, {:?}", same_team, opp_team);
    if same_team.is_empty() && opp_team.is_empty() {
        return load_all_matches(&champions, pool);
    }
    let conn = pool.get().unwrap();
    let mut same_ids: Vec<i16> = Vec::new();
    for name in same_team {
        same_ids.push(champions.id_from_name(&name));
    }

    let mut opp_ids: Vec<i16> = Vec::new();
    for name in opp_team {
        opp_ids.push(champions.id_from_name(&name));    
    }
    let mut matches: Vec<GlobalMatch> = Vec::new();
    for row in &conn.query(Q_GLOBAL_MATCHES_BOTH_TEAM_BLUE, &[&same_ids, &opp_ids])? {
        let same_champ_ids = row.get(1);
        let opp_champ_ids = row.get(2);
        let same_bans = row.get(3);
        let opp_bans = row.get(4);
        let m = GlobalMatch {
            same_wins: row.get(0),
            same_team: champions.idxs_from_ids(&same_champ_ids),
            opp_team: champions.idxs_from_ids(&opp_champ_ids),
            same_bans: champions.idxs_from_ids(&same_bans),
            opp_bans: champions.idxs_from_ids(&opp_bans)
        };
        matches.push(m);
    }
    for row in &conn.query(Q_GLOBAL_MATCHES_BOTH_TEAM_RED, &[&same_ids, &opp_ids])? {
        let opp_wins: bool = row.get(0);
        let same_champ_ids = row.get(2);
        let opp_champ_ids = row.get(1);
        let same_bans = row.get(4);
        let opp_bans = row.get(3);
        let m = GlobalMatch {
            same_wins: !opp_wins,
            same_team: champions.idxs_from_ids(&same_champ_ids),
            opp_team: champions.idxs_from_ids(&opp_champ_ids),
            same_bans: champions.idxs_from_ids(&same_bans),
            opp_bans: champions.idxs_from_ids(&opp_bans)
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
fn load_all_matches(champions: &Champions, pool: ConnectionPool) -> Result<Vec<GlobalMatch>, Error> {
    let conn = pool.get().unwrap();
    let mut matches: Vec<GlobalMatch> = Vec::new();
    for row in &conn.query(Q_ALL_MATCHES, &[])? {
        let blue_wins: bool = row.get(0);
        let blue_champ_ids = row.get(1);
        let red_champ_ids = row.get(2);
        let blue_bans = row.get(3);
        let red_bans = row.get(4);
        let m = GlobalMatch {
            same_wins: blue_wins,
            same_team: champions.idxs_from_ids(&blue_champ_ids),
            opp_team: champions.idxs_from_ids(&red_champ_ids),
            same_bans: champions.idxs_from_ids(&blue_bans),
            opp_bans: champions.idxs_from_ids(&red_bans)
        };
        matches.push(m);
        let m_flipped = GlobalMatch {
            same_wins: !blue_wins,
            same_team: champions.idxs_from_ids(&red_champ_ids),
            opp_team: champions.idxs_from_ids(&blue_champ_ids),
            same_bans: champions.idxs_from_ids(&red_bans),
            opp_bans: champions.idxs_from_ids(&blue_bans)
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