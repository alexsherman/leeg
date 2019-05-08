/**
 * Contains the main workhorse functions for generating champion recommendations
 * @author dmcfalls
 */

use champions::Champions;
use matches::Match;
use scores::ScoreMatrix;
use scores::SimpleIndependentScoreMatrix;

use utils::argmax::argmax_idx;

// Used for comparisons. The "empty product" is 1 instead of 0, but 1 is greater than any
// score we'd encounter, so we filter out any value of 1.
const ONE_F64: f64 = 0.999999999;
const ZERO_F64: f64 = 0.0;

/**
 * A champion recommendation service. Must implement a method that, given the player's team's
 * picks, the picks on the opposing team, and num_reqs, returns num_reqs champions as
 * suggested picks for that player.
 */
pub trait ReqService {

	/**
	 * Given two vectors of champion names and a usize, returns usize champion recommendations.
	 */
	fn req(&self, team_picks: &Vec<String>, opp_picks: &Vec<String>, num_reqs: usize)
			-> Vec<String>;

}

 /**
  * Champion recommendation service for a single summoner using that summoner's match history
  */
pub struct SingleSummonerReqService {
	champions: Champions,
	score_matrix: SimpleIndependentScoreMatrix
}

impl SingleSummonerReqService {

	pub fn from_matches(matches: &Vec<Match>, champions: &Champions) -> SingleSummonerReqService {
		return SingleSummonerReqService {
			champions: champions.clone(),
			score_matrix: SimpleIndependentScoreMatrix::from_matches(matches, champions.len())
		};
	}

}

impl ReqService for SingleSummonerReqService {

	fn req(&self, team_picks: &Vec<String>, opp_picks: &Vec<String>, num_reqs: usize)
			-> Vec<String> {
		
		let team_idxs = self.champions.idxs_from_names(team_picks);
		let opp_idxs = self.champions.idxs_from_names(opp_picks);

		// Given team_idxs and opp_idxs, calculate scores for all champions
		let mut scores: Vec<f64> = Vec::with_capacity(self.champions.len());
		for i in 0..self.champions.len() {
			let mut score_i: f64 = self.score_matrix.ally_score_product(&i, &team_idxs)
					* self.score_matrix.opp_score_product(&i, &opp_idxs);
			if score_i > ONE_F64 {
				score_i = ZERO_F64;
			}		
			scores.push(score_i);
		}

		// Now, find the highest order statistics in scores and return them as the recommendations
		let mut req_idxs: Vec<usize> = Vec::new();
		for _ in 0..num_reqs {
			let req_idx = argmax_idx(&scores);
			req_idxs.push(req_idx);
			// Zero-out the score in question so that reqs are not duplicated
			scores[req_idx] = ZERO_F64;
		}

		return self.champions.names_from_idxs(&req_idxs);
	}

}