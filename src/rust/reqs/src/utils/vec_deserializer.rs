/**
 * Utility functions for deserializing vectors from string data
 */

use utils::string::strip_square_brackets;
use utils::string::strip_outer_quotes;

/**
 * Converts a string-representation of a python list into a Vec<String>
 */
pub fn vec_from_python_list(pylist: &String) -> Vec<String> {
	let list = strip_square_brackets(pylist);
	let split = list.split(',');
	let mut result = Vec::new();
	for s in split {
		result.push(strip_outer_quotes(&s.trim().to_string()));
	}
	return result;
}


#[cfg(test)]
mod tests {

use utils::vec_deserializer::vec_from_python_list;

	#[test]
	fn test_vec_from_python_list() {
		let input: String = "['Miss Fortune', 'Wukong', 'Ezreal', 'Malphite']".to_string();
		let expected: Vec<String> = vec![
				"Miss Fortune".to_string(),
				"Wukong".to_string(),
				"Ezreal".to_string(),
				"Malphite".to_string()
		];
		assert_eq!(expected, vec_from_python_list(&input));
	}

}