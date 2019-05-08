/**
 * Contains static champion data and functions to load that data from JSON
 * @author dmcfalls
 */

extern crate serde_json;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

// Used as a hint to pre-allocate data structures correlated to the set of champions
pub const EXPECTED_CHAMPIONS_COUNT: usize = 143;

/**
 * A single champion
 */
#[derive(Debug, Clone, Deserialize)]
pub struct Champion {
	name: String,
	id: i16
}

impl PartialEq for Champion {
	fn eq(&self, other: &Champion) -> bool {
		self.name == other.name && self.id == other.id
	}
}

/**
 * Convenience type for the reference data for all champions
 */
#[derive(Debug, Clone)]
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

	pub fn len(&self) -> usize {
		return self.list.len();
	}

	pub fn idxs_from_names(&self, champion_names: &Vec<String>) -> Vec<usize> {
		return champion_names.iter()
				.map(|name| self.name_hashes.get(name).unwrap().clone())
				.collect();
	}

	pub fn names_from_idxs(&self, champion_idxs: &Vec<usize>) -> Vec<String> {
		return champion_idxs.iter()
				.map(|idx| self.list[*idx].name.clone())
				.collect();
	}

	fn push(&mut self, id: i16, name: String) {
		let champion = Champion {name: name, id: id};
		let idx = self.list.len();
		self.id_hashes.insert(id, idx);
		self.name_hashes.insert(champion.name.clone(), idx);
		self.list.push(champion);
	}

}

/**
 * Loads a Champions vector from JSON file data
 */
pub fn load_champions(filename: String) -> Champions {
	let mut json_file = File::open(filename).expect("Unable to open file");
	let mut raw_json = String::new();
	json_file.read_to_string(&mut raw_json).expect("Unable to read file");

	let champs_map : HashMap<i16, String> = serde_json::from_str(&raw_json).unwrap();
	let mut champions = Champions::new();
	for (id, name) in champs_map {
		champions.push(id, name);
	}

	return champions;
}