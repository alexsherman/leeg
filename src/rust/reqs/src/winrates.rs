/**
 * Structs and traits for representing win-rate matrices
 * @author dmcfalls
 */

 /**
  * Represents a 2d n*n array of champions and associated winrates
  */
struct Winrates {
	matrix: Vec<Vec<f64>>
}