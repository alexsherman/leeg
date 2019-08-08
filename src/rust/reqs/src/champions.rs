/**
 * Contains static champion data and functions to load that data from JSON
 * @author dmcfalls, alexsherman
 */
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde_json;
extern crate postgres;

use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use std::collections::HashMap;
use utils::postgres_utils::*;
use self::postgres::Error;

// Used as a hint to pre-allocate data structures correlated to the set of champions
pub const EXPECTED_CHAMPIONS_COUNT: usize = 143;

/**
 * A single champion
 */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Champion {
	pub name: String,
	id: i16,
	roles: Vec<String>
}

impl Champion {
	pub fn get_roles(&self) -> &Vec<String> {
		&self.roles
	}
}

impl PartialEq for Champion {
	fn eq(&self, other: &Champion) -> bool {
		self.name == other.name && self.id == other.id
	}
}

/**
 * Convenience type for the reference data for all champions
 */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Champions {
	list: Vec<Champion>,
	id_hashes: HashMap<i16, usize>,
	name_hashes: HashMap<String, usize>
}

impl Champions {

	pub fn new() -> Champions {
		Champions {
			list: Vec::with_capacity(EXPECTED_CHAMPIONS_COUNT),
			id_hashes: HashMap::with_capacity(EXPECTED_CHAMPIONS_COUNT),
			name_hashes: HashMap::with_capacity(EXPECTED_CHAMPIONS_COUNT)
		}
	}

	pub fn index_by_name(&self, name: &String) -> &usize {
		return &self.name_hashes.get(name).unwrap();
	}

	pub fn index_by_id(&self, id: &i16) -> &usize {
		return &self.id_hashes.get(id).unwrap();
	}

	pub fn len(&self) -> usize {
		return self.list.len();
	}

	pub fn idxs_from_names(&self, champion_names: &Vec<String>) -> Vec<usize> {
		let mut ids: Vec<usize> = Vec::new();
		for name in champion_names {
			match self.name_hashes.get(name) {
				Some(n) => {
					ids.push(n.clone());
				},
				None => ()
			};
		}
		ids
	}

	pub fn idxs_from_ids(&self, champion_ids: &Vec<i16>) -> Vec<usize> {
		let mut ids: Vec<usize> = Vec::new();
		for id in champion_ids {
			match self.id_hashes.get(id) {
				Some(i) => {
					ids.push(i.clone());
				},
				None => ()
			};
		}
		ids
	}

	pub fn names_from_idxs(&self, champion_idxs: &Vec<usize>) -> Vec<String> {
		return champion_idxs.iter()
				.map(|idx| self.list[*idx].name.clone())
				.collect();
	}

	pub fn get_list(&self) -> &Vec<Champion> {
		&self.list
	}

	pub fn name_from_id(&self, id: &i16) -> String {
		let idx = *self.index_by_id(id);
		self.list[idx].name.clone()
	}

	pub fn id_from_name(&self, name: &String) -> i16 {
		let idx = *self.index_by_name(name);
		self.list[idx].id
	}

	fn push(&mut self, id: i16, name: String, roles: Vec<String>) {
		let champion = Champion {name: name, id: id, roles: roles};
		let idx = self.list.len();
		self.id_hashes.insert(id, idx);
		self.name_hashes.insert(champion.name.clone(), idx);
		self.list.push(champion);
	}

}

pub fn load_champions_from_db(pool: Pool<PostgresConnectionManager>) -> Result<Champions, Error> {
 	let conn = pool.get().unwrap();
 	let mut champions = Champions::new();
    for row in &conn.query(Q_ALL_CHAMPIONS, &[])? {
		champions.push(row.get(0), row.get(1), row.get(2));
    }
    Ok(champions)
}