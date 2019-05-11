/**
 * Structs and traits for representing win-rate matrices
 * @author dmcfalls
 */

use matches::Match;

/**
 * Represents a score indicating how well one champion fares against another.
 * The "meaning" of a Score is determined by the structure using it,
 * but a higher score is generally better than a lower one.
 */
type Score = f64;

/**
 * A data structure containing scores. There is no enforcement that it organize
 * data in any way, only that it implements a constructor from match data and dimensions
 */
pub trait ScoreMatrix {

	fn from_matches(matches: &Vec<Match>, n: usize) -> Self;

}

 /**
  * Represents relative scores among champions generated using simple independent win-rates
  */
#[derive(Debug)]  
pub struct SimpleIndependentScoreMatrix {
	ally_scores_2d: Vec<Vec<Score>>,
	opp_scores_2d: Vec<Vec<Score>>,
	n: usize
}

impl ScoreMatrix for SimpleIndependentScoreMatrix {

	fn from_matches(matches: &Vec<Match>, n: usize) -> SimpleIndependentScoreMatrix {
		let mut score_matrix = SimpleIndependentScoreMatrix::with_dimensions(n);
		let mut match_counts = MatchCounts::with_dimensions(n);
		for m in matches {
			match_counts.populate_match_data(m);
		}
		score_matrix.get_scores_from_match_counts(&match_counts);
		return score_matrix;
	}

}

impl SimpleIndependentScoreMatrix {

	pub fn ally_score_product(&self, champ_idx: &usize, ally_idxs: &Vec<usize>) -> f64 {
		return ally_idxs.iter()
				.map(|idx| self.ally_scores_2d[*champ_idx][*idx])
				.filter(|score| *score > 0f64)
				.product();
	}

	pub fn opp_score_product(&self, champ_idx: &usize, opp_idxs: &Vec<usize>) -> f64 {
		return opp_idxs.iter()
				.map(|idx| self.opp_scores_2d[*champ_idx][*idx])
				.filter(|score| *score > 0f64)
				.product();
	}

	fn with_dimensions(n: usize) -> SimpleIndependentScoreMatrix {
		let mut score_matrix = SimpleIndependentScoreMatrix {
			ally_scores_2d: Vec::with_capacity(n),
			opp_scores_2d: Vec::with_capacity(n),
			n: n
		};
		for _ in 0..n {
			score_matrix.ally_scores_2d.push(vec![0f64; n]);
			score_matrix.opp_scores_2d.push(vec![0f64; n]);
		}
		return score_matrix;
	}

	fn get_scores_from_match_counts(&mut self, match_counts: &MatchCounts) {
		for i in 0..self.n {
			for j in 0..self.n {
				self.ally_scores_2d[i][j]
						= match_counts.ally_wins[i][j] as f64 / match_counts.ally_games[i][j] as f64;
				self.opp_scores_2d[i][j]
						= match_counts.vs_wins[i][j] as f64 / match_counts.vs_games[i][j] as f64;
			}
		}
	}

}

/**
 * Helper struct to store intermediate values before converting to a Scores
 */
#[derive(Debug)]
struct MatchCounts {
	ally_games: Vec<Vec<u32>>,
	ally_wins: Vec<Vec<u32>>,
	vs_games: Vec<Vec<u32>>,
	vs_wins: Vec<Vec<u32>>
}

impl MatchCounts {

	fn with_dimensions(n: usize) -> MatchCounts {
		let mut match_counts = MatchCounts {
			ally_games: Vec::with_capacity(n),
			ally_wins: Vec::with_capacity(n),
			vs_games: Vec::with_capacity(n),
			vs_wins: Vec::with_capacity(n)
		};
		for _ in 0..n {
			match_counts.ally_games.push(vec![0u32; n]);
			match_counts.ally_wins.push(vec![0u32; n]);
			match_counts.vs_games.push(vec![0u32; n]);
			match_counts.vs_wins.push(vec![0u32; n]);
		}
		return match_counts;
	}

	fn populate_match_data(&mut self, m: &Match) {
		let summoner_champ_idx = m.get_summoner_champion_idx();
		let win_increment = match m.is_summoner_win() {
			true => 1,
			false => 0
		};
		for champ_idx in m.get_same_team_champion_idxs() {
			self.ally_games[summoner_champ_idx][*champ_idx] += 1;
			self.ally_wins[summoner_champ_idx][*champ_idx] += win_increment;
		}
		for champ_idx in m.get_opposing_team_champion_idxs() {
			self.vs_games[summoner_champ_idx][*champ_idx] += 1;
			self.vs_wins[summoner_champ_idx][*champ_idx] += win_increment;
		}
	}

}