/**
 * Contains function to deserialize bool type from String
 */

/**
 * Parses a bool from String
 * 
 * TODO: this function should be made more robust, and ideally should return a Result<bool>
 */
pub fn bool_from_string(string: &String) -> bool {
	let lower = string.to_lowercase();
	return lower == "true";
}