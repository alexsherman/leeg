/**
 * Contains the main workhorse functions for generating champion recommendations
 * @author dmcfalls
 * @author alexsherman
 */

use champions::Champions;
use matches::{GlobalMatch};
use scores::{ScoreVector, GlobalScoreVectors};

use utils::argmax::argmax_idx;

// Used for comparisons. The "empty product" is 1 instead of 0, but 1 is greater than any
// score we'd encounter, so we filter out any value of 1.
const ONE_F64: f64 = 1f64;
const ZERO_F64: f64 = 0f64;

/**
 * Champion recommendation service constructed for a particular partial team comp/matchup.
 */
 #[derive(Clone, Deserialize, Serialize)]
pub struct GlobalReqService {
    team_picks: Vec<String>,
    opp_picks: Vec<String>,
    score_vectors: GlobalScoreVectors,
}

impl GlobalReqService {
   	pub fn req_banless(&self, champions: &Champions, num_reqs: usize)
	       		 -> Vec<String> {
	    
	    let mut scores = self.calculate_scores(&champions);
	    return self.get_reqs(&mut scores, num_reqs, &champions);
    
    } 

	/**
	 * Construct a new GlobalReqService given match and champion data
	 */
	pub fn from_matches(matches: &Vec<GlobalMatch>, team_picks: &Vec<String>, opp_picks: &Vec<String>, num_champions: usize) -> GlobalReqService {
		return GlobalReqService {
			team_picks: team_picks.clone(),
			opp_picks: opp_picks.clone(),
			score_vectors: GlobalScoreVectors::from_global_matches(matches, num_champions),
		};
	}

	/**
	 * Currently only calculates the best picks for someone on the "same" team. Could also return
     * results for "opp" team, representing the best picks against
	 */
	fn calculate_scores(&self, champions: &Champions) -> Vec<f64> {
        let team_idxs = champions.idxs_from_names(&self.team_picks);
        let opp_idxs = champions.idxs_from_names(&self.opp_picks);

		let mut scores: Vec<f64> = Vec::with_capacity(champions.len());
		for i in 0..champions.len() { 
			let mut score_i: f64 = self.score_vectors.same_winrates[i]; 
			if score_i == ONE_F64 || team_idxs.contains(&i) || opp_idxs.contains(&i) {
				score_i = ZERO_F64;
			}
			scores.push(score_i);
		}
		scores
	}

    fn get_reqs(&self, scores: &mut Vec<f64>, num_reqs: usize, champions: &Champions) -> Vec<String> {
		let mut req_idxs: Vec<usize> = Vec::new();
        let mut top_scores: Vec<f64> = Vec::new();
        let mut pick_rates: Vec<f64> = Vec::new();
        for _ in 0..num_reqs {
			let req_idx = argmax_idx(&scores);
			if scores[req_idx] == ZERO_F64 {
				break;
			}
			req_idxs.push(req_idx);
            top_scores.push(scores[req_idx]);
            pick_rates.push(self.score_vectors.same_pickrates[req_idx] + self.score_vectors.opp_pickrates[req_idx]);
			// Zero-out the score in question so that reqs are not duplicated
			scores[req_idx] = ZERO_F64;
		}
        let names = champions.names_from_idxs(&req_idxs);
        for i in 0..top_scores.len() {
            println!("{} -> {:.2}% (picked {:.2}%)", names[i], top_scores[i] * 100f64, pick_rates[i] * 100f64);
        }
        names
	}


}

#[derive(Clone, Deserialize, Serialize)]
pub struct GlobalServiceWithWeight {
	pub req_service: GlobalReqService,
	pub weight: usize
}

#[derive(Clone, Deserialize, Serialize)]
pub struct NamedGlobalService {
	pub req_service: GlobalReqService,
	pub champ_name: String
}

/**
*  Given a vector of GlobalServiceWithWeights which each have a GlobalReqService and a weight,
* combines the scores of the vectors given their respective weights.
* Note that champion vectors should be in the same order for all GSWW.
*/ 
pub fn combine_req_services(services: &Vec<GlobalServiceWithWeight>, team_picks: &Vec<String>, 
							opp_picks: &Vec<String>, roles: Option<Vec<String>>, champions: &Champions) -> GlobalReqService {
	let mut combined_service = GlobalReqService {
		team_picks: team_picks.clone(),
		opp_picks: opp_picks.clone(),
		score_vectors: GlobalScoreVectors::with_dimensions(champions.len()),
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
			combined_service.score_vectors.opp_winrates[i] += adjusted_weight * s.req_service.score_vectors.opp_winrates[i];
			combined_service.score_vectors.same_pickrates[i] += adjusted_weight * s.req_service.score_vectors.same_pickrates[i];
			combined_service.score_vectors.opp_pickrates[i] += adjusted_weight * s.req_service.score_vectors.opp_pickrates[i];
			combined_service.score_vectors.same_banrates[i] += adjusted_weight * s.req_service.score_vectors.same_banrates[i];
			combined_service.score_vectors.opp_banrates[i] += adjusted_weight * s.req_service.score_vectors.opp_banrates[i];
		}
	}
	// filter out champs which don't match the roles specified
	for i in 0..champions.len() { 
		let champ_matches_roles: bool = match &roles {
			Some(x) => {
				let mut found = false;
				for role in x {
					if champions.get_list()[i].get_roles().contains(&role) {
						found = true;
						break;
					}
				}
				found
			},
			None => true
		};
		if !champ_matches_roles {
			combined_service.score_vectors.same_winrates[i] = ZERO_F64;
		}
	}
	combined_service
}
