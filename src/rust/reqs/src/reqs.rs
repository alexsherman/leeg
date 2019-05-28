/**
 * Contains the main workhorse functions for generating champion recommendations
 * @author dmcfalls
 */

use champions::Champions;
use matches::{Match, GlobalMatch};
use scores::{ScoreMatrix, ScoreVector};
use scores::{SimpleIndependentScoreMatrix, GlobalScoreVectors};

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

/**
 * Champion recommendation service constructed for a particular partial team comp/matchup.
 */
pub struct GlobalReqService {
    champions: Champions,
    score_vectors: GlobalScoreVectors,
}

impl ReqService for GlobalReqService {

    fn req(&self, team_picks: &Vec<String>, opp_picks: &Vec<String>,
            team_bans: &Vec<String>, opp_bans: &Vec<String>, num_reqs: usize)
            -> Vec<String> {
        println!("unimplemented rn!");
        let empty: Vec<String> = Vec::new();
        empty
    }

    fn req_banless(&self, team_picks: &Vec<String>, opp_picks: &Vec<String>, num_reqs: usize)
            -> Vec<String> {
        
        let mut scores = self.calculate_scores(team_picks, opp_picks);
        return self.get_reqs(&mut scores, num_reqs);
        
    } 
}

impl GlobalReqService {

	/**
	 * Construct a new GlobalReqService given match and champion data
	 */
	pub fn from_matches(matches: &Vec<GlobalMatch>, champions: &Champions) -> GlobalReqService {
		return GlobalReqService {
			champions: champions.clone(),
			score_vectors: GlobalScoreVectors::from_global_matches(matches, champions.len()),
		};
	}

	/**
	 * Currently only calculates the best picks for someone on the "same" team. Could also return
     * results for "opp" team, representing the best picks against
	 */
	fn calculate_scores(&self, team_picks: &Vec<String>, opp_picks: &Vec<String>) -> Vec<f64> {
        let team_idxs = self.champions.idxs_from_names(team_picks);
        let opp_idxs = self.champions.idxs_from_names(opp_picks);

		let mut scores: Vec<f64> = Vec::with_capacity(self.champions.len());
		for i in 0..self.champions.len() { 
			let mut score_i: f64 = self.score_vectors.same_winrates[i]; 
			if score_i == ONE_F64 || team_idxs.contains(&i) || opp_idxs.contains(&i) {
				score_i = ZERO_F64;
			}
			scores.push(score_i);
		}
		scores
	}

    fn get_reqs(&self, scores: &mut Vec<f64>, num_reqs: usize) -> Vec<String> {
		let mut req_idxs: Vec<usize> = Vec::new();
        let mut top_scores: Vec<f64> = Vec::new();
        for _ in 0..num_reqs {
			let req_idx = argmax_idx(&scores);
			req_idxs.push(req_idx);
            top_scores.push(scores[req_idx]);
			// Zero-out the score in question so that reqs are not duplicated
			scores[req_idx] = ZERO_F64;
		}
        let names = self.champions.names_from_idxs(&req_idxs);
        for i in 0..num_reqs {
            println!("{} -> {}", names[i], top_scores[i]);
        }
        names
	}


}

pub struct GlobalServiceWithWeight {
	pub req_service: GlobalReqService,
	pub weight: usize
}

/**
*  Given a vector of GlobalServiceWithWeights which each have a GlobalReqService and a weight,
* combines the scores of the vectors given their respective weights.
* Note that champion vectors should be in the same order for all GSWW.
*/ 
pub fn combine_req_services(services: &Vec<GlobalServiceWithWeight>, roles: Option<Vec<String>>) -> GlobalReqService {
	let mut combined_service = GlobalReqService {
		champions: services[0].req_service.champions.clone(),
		score_vectors: GlobalScoreVectors::with_dimensions(services[0].req_service.champions.len()),
	};

	let mut total_weight: usize = 0;
	for s in services {
		total_weight += s.weight;
	}

	for s in services {
		let adjusted_weight = s.weight as f64 / total_weight as f64;
		println!("Weight: {}", adjusted_weight);
		for i in 0..s.req_service.score_vectors.same_winrates.len() {
			combined_service.score_vectors.same_winrates[i] += adjusted_weight * s.req_service.score_vectors.same_winrates[i];
		}
	}
	// filter out champs which don't match the roles specified
	for i in 0..combined_service.champions.len() { 
		let champ_matches_roles: bool = match &roles {
			Some(x) => {
				let mut found = false;
				for role in x {
					if combined_service.champions.get_list()[i].get_roles().contains(&role) {
						found = true;
					}
				}
				found
			},
			None => true
		};
		if !champ_matches_roles {
			println!("eliminating champ based on role");
			combined_service.score_vectors.same_winrates[i] = ZERO_F64;
		}
	}
	combined_service
}
