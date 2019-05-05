/**
 * Contains static champion data and functions to load that data from JSON
 * @author dmcfalls
 */

extern crate serde_json;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

const INVALID_CHAMPION_ID: i16 = -1;
const EXPCETED_CHAMPIONS_COUNT: usize = 143;

/**
 * A single champion
 */
 #[derive(Debug, Clone, Deserialize)]
pub struct Champion {
	name: String,
	id: i16
}

impl Champion {

	fn new() -> Champion {
		Champion { name: "".to_string(), id: INVALID_CHAMPION_ID }
	}

	fn to_string(&self) -> String {
		return self.name.to_string();
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
#[derive(Debug)]
pub struct Champions {
	list: Vec<Champion>
}

impl Champions {

	pub fn new() -> Champions {
		Champions { list: Vec::with_capacity(EXPCETED_CHAMPIONS_COUNT) }
	}

	pub fn index_by_name(&self, name: String) -> Option<usize> {
		return self.list.iter().position(|champ| champ.name == name);
	}

	pub fn by_id(&self, id: i16) -> &Champion {
		if id == INVALID_CHAMPION_ID || self.list[id as usize].id == INVALID_CHAMPION_ID {
			panic!("Invalid champion id specified in call to Champions::by_id");
		}
		return &self.list[id as usize];
	}

	pub fn count(&self) -> usize {
		let mut count: usize = 0;
		for champion in self.list.iter() {
			if champion.id != INVALID_CHAMPION_ID {
				count += 1;
			}
		}
		return count;
	}

	pub fn len(&self) -> usize {
		return self.list.len();
	}

	fn add(&mut self, id: i16, name: String) {
		let champion = Champion {name: name, id: id};
		if (id as usize) >= self.list.len() {
			self.list.resize(id as usize, Champion::new());
		}
		self.list.insert(id as usize, champion);
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
		champions.add(id, name);
	}

	return champions;
}