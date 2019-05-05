/**
 * Contains static champion data and functions to load that data from JSON
 * @author dmcfalls
 */

extern crate serde_json;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

/**
 * A single champion
 */
 #[derive(Debug)]
pub struct Champion {
	name: String,
}

impl Champion {

	fn to_string(&self) -> String {
		return self.name.to_string();
	}

}

impl PartialEq for Champion {
	fn eq(&self, other: &Champion) -> bool {
		self.name == other.name
	}
}

/**
 * Convenience type for the reference data for all champions
 */
#[derive(Debug)]
pub struct Champions {
	list: Vec<Champion>
}

impl Champions {

	fn new() -> Champions {
		Champions { list: Vec::new() }
	}

	fn index_by_name(&self, name: String) -> Option<usize> {
		return self.list.iter().position(|champ| champ.name == name);
	}

	fn add(&mut self, name: String) {
		let champion = Champion {name: name};
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

	let champs_map : HashMap<i32, String> = serde_json::from_str(&raw_json).unwrap();
	let mut champions = Champions::new();
	for (_, name) in champs_map.iter() {
		champions.add(name.to_string());
	}

	return champions;
}