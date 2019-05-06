/**
 * String utils speciailized for our purposes, not meant to be generalizable
 */

pub fn strip_square_brackets(string: &String) -> String {
	string.replace("[", "").replace("]", "")
}

pub fn strip_outer_quotes(string: &String) -> String {
	let end = string.len() - 1;
	if string.chars().nth(0).unwrap() == '\'' && string.chars().nth(end).unwrap() == '\'' {
		return string[1..end].to_string();
	}
	if string.chars().nth(0).unwrap() == '\"' && string.chars().nth(end).unwrap() == '\"' {
		return string[1..end].to_string();
	}
	return string.clone();
}


#[cfg(test)]
mod tests {

use utils::string::strip_square_brackets;
use utils::string::strip_outer_quotes;

	#[test]
	fn test_strip_square_brackets() {
		let input: String = "['Lux', 'Wukong']".to_string();
		assert_eq!("'Lux', 'Wukong'".to_string(), strip_square_brackets(input));
	}
	
	#[test]
	fn test_strip_outer_quotes() {
		let input: String = "'Lux'".to_string();
		assert_eq!("Lux".to_string(), strip_outer_quotes(input));
	}

	#[test]
	fn test_strip_outer_quotes_preserve_apostrophe() {
		let input: String = "'Rek'Sai'".to_string();
		assert_eq!("Rek'Sai".to_string(), strip_outer_quotes(input));
	}

}
