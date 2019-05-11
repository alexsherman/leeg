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
const ONE_F64: f64 = 1f64;
const ZERO_F64: f64 = 0f64;

/**
 * A champion recommendation service. Must implement a method that, given the player's team's
 * picks, the picks on the opposing team, and num_reqs, returns num_reqs champions as
 * suggested picks for that player.
 */
pub trait ReqService {

	/**
	 * Given a usize, two vectors of champion names representing the two teams' picks and
	 * two additional vectors of champion names representing the two teams' bans, returns
	 * num_reqs champion recommendations.
	 */
	fn req(&self, team_picks: &Vec<String>, opp_picks: &Vec<String>,
			team_bans: &Vec<String>, opp_bans: &Vec<String>, num_reqs: usize)
			-> Vec<String>;

	/**
	 * Given two vectors of champion names and a usize, returns num_reqs champion recommendations.
	 * This variant includes no notion of bans.
	 */
	fn req_banless(&self, team_picks: &Vec<String>, opp_picks: &Vec<String>, num_reqs: usize)
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

	/**
	 * Construct a new SingleSummonerReqService given match and champion data
	 */
	pub fn from_matches(matches: &Vec<Match>, champions: &Champions) -> SingleSummonerReqService {
		return SingleSummonerReqService {
			champions: champions.clone(),
			score_matrix: SimpleIndependentScoreMatrix::from_matches(matches, champions.len())
		};
	}

	/**
	 * Iterate through match data and assign a score for each champion
	 */
	fn calculate_scores(&self, team_picks: &Vec<String>, opp_picks: &Vec<String>) -> Vec<f64> {
		let team_idxs = self.champions.idxs_from_names(team_picks);
		let opp_idxs = self.champions.idxs_from_names(opp_picks);

		let mut scores: Vec<f64> = Vec::with_capacity(self.champions.len());
		for i in 0..self.champions.len() {
			let mut score_i: f64 = self.score_matrix.champ_winrate(&i)
					* self.score_matrix.ally_score_product(&i, &team_idxs)
					* self.score_matrix.opp_score_product(&i, &opp_idxs);
			if score_i == ONE_F64 {
				score_i = ZERO_F64;
			}
			scores.push(score_i);
		}
		return scores;
	}

	/**
	 * Set scores for banned champions to zero since they are not able to be picked
	 */
	fn filter_champs(&self, scores: &mut Vec<f64>, champs1: &Vec<String>, champs2: &Vec<String>) {
		
		let champs1_idxs = self.champions.idxs_from_names(champs1);
		let champs2_idxs = self.champions.idxs_from_names(champs2);

		for (idx, score) in scores.iter_mut().enumerate() {
			if champs1_idxs.contains(&idx) || champs2_idxs.contains(&idx) {
				*score = ZERO_F64;
			}
		}

	}

	/**
	 * Find the highest order statistics in scores and return them as the recommendations
	 */
	fn get_reqs(&self, scores: &mut Vec<f64>, num_reqs: usize) -> Vec<String> {
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

impl ReqService for SingleSummonerReqService {

	fn req(&self, team_picks: &Vec<String>, opp_picks: &Vec<String>,
			team_bans: &Vec<String>, opp_bans: &Vec<String>, num_reqs: usize)
			-> Vec<String> {

		let mut scores = self.calculate_scores(team_picks, opp_picks);
		self.filter_champs(&mut scores, team_bans, opp_bans);
		self.filter_champs(&mut scores, team_picks, opp_picks);
		return self.get_reqs(&mut scores, num_reqs);

	}

	fn req_banless(&self, team_picks: &Vec<String>, opp_picks: &Vec<String>, num_reqs: usize)
			-> Vec<String> {
		
		let mut scores = self.calculate_scores(team_picks, opp_picks);
		self.filter_champs(&mut scores, team_picks, opp_picks);
		return self.get_reqs(&mut scores, num_reqs);
		
	}

}