/**
 * Utility functions for finding argmax in a collection
 */

/**
 * Specialized argmax function to get the index of the maximum f64 in a vector
 */
pub fn argmax_idx(floats: &Vec<f64>) -> usize {
	let mut max: f64 = 0.0;
	let mut result: usize = 0;
	for i in 0..floats.len() {
		if floats[i] >= max {
			max = floats[i];
			result = i;
		}
	}
	return result;
}

#[cfg(test)]
mod tests {

	use utils::argmax::argmax_idx;

	#[test]
	fn test_argmax_idx() {
		let input: Vec<f64> = vec![1.0, 2.0, 1.5, 6.5, 4.5, 10.0, 5.0, 11.0, 5.0, 1.0];
		assert_eq!(7, argmax_idx(&input));
	}

	#[test]
	fn test_argmax_idx_returns_last() {
		let input: Vec<f64> = vec![1.7, 2.0, 3.0, 1.2, 3.0, 1.5];
		assert_eq!(4, argmax_idx(&input));
	}

}