/**
 * Structs and traits for representing win-rate matrices
 * @author dmcfalls
 */

use matches::Match;

use champions::EXPECTED_CHAMPIONS_COUNT;

/**
 * Represents a score indicating how well one champion fares against another.
 * The "meaning" of a Score is determined by the structure using it,
 * but a higher score is generally better than a lower one.
 */
type Score = f64;

/**
 * A data structure containing scores. There is no enforcement that it organize
 * data in any way, only that it implements a constructor from match data.
 */
pub trait ScoreMatrix {

	fn from_matches(matches: Vec<Match>) -> Self;

}

 /**
  * Represents relative scores among champions generated using simple independent win-rates
  */
pub struct SimpleIndependentScoreMatrix {
	ally_scores_2d: Vec<Vec<Score>>,
	opp_scores_2d: Vec<Vec<Score>>
}

impl ScoreMatrix for SimpleIndependentScoreMatrix {

	fn from_matches(matches: Vec<Match>) -> SimpleIndependentScoreMatrix {
		let mut score_matrix = SimpleIndependentScoreMatrix::new();
		let mut match_counts = MatchCounts::new();
		for m in matches {
			match_counts.populate_match_data(m);
		}
		score_matrix.get_scores_from_match_counts(match_counts);
		return score_matrix;
	}

}

impl SimpleIndependentScoreMatrix {

	pub fn new() -> SimpleIndependentScoreMatrix {
		// Initializes the score matrix with nested capacity sufficient for all champions
		let mut score_matrix = SimpleIndependentScoreMatrix {
			ally_scores_2d: Vec::with_capacity(EXPECTED_CHAMPIONS_COUNT), 
			opp_scores_2d: Vec::with_capacity(EXPECTED_CHAMPIONS_COUNT)
		};
		for _ in 0..EXPECTED_CHAMPIONS_COUNT {
			score_matrix.ally_scores_2d.push(Vec::with_capacity(EXPECTED_CHAMPIONS_COUNT));
			score_matrix.opp_scores_2d.push(Vec::with_capacity(EXPECTED_CHAMPIONS_COUNT));
		}
		return score_matrix;
	}

	fn get_scores_from_match_counts(&mut self, match_counts: MatchCounts) {
		// TODO: implement
	}

}

/**
 * Helper struct to store intermediate values before converting to a Scores
 */
struct MatchCounts {
	ally_games: Vec<Vec<u32>>,
	ally_wins: Vec<Vec<u32>>,
	vs_games: Vec<Vec<u32>>,
	vs_wins: Vec<Vec<u32>>
}

impl MatchCounts {

	fn new() -> MatchCounts {
		let mut match_counts = MatchCounts {
			ally_games: Vec::with_capacity(EXPECTED_CHAMPIONS_COUNT),
			ally_wins: Vec::with_capacity(EXPECTED_CHAMPIONS_COUNT),
			vs_games: Vec::with_capacity(EXPECTED_CHAMPIONS_COUNT),
			vs_wins: Vec::with_capacity(EXPECTED_CHAMPIONS_COUNT)
		};
		for _ in 0..EXPECTED_CHAMPIONS_COUNT {
			match_counts.ally_games.push(Vec::with_capacity(EXPECTED_CHAMPIONS_COUNT));
			match_counts.ally_wins.push(Vec::with_capacity(EXPECTED_CHAMPIONS_COUNT));
			match_counts.vs_games.push(Vec::with_capacity(EXPECTED_CHAMPIONS_COUNT));
			match_counts.vs_wins.push(Vec::with_capacity(EXPECTED_CHAMPIONS_COUNT));
		}
		return match_counts;
	}

	fn populate_match_data(&mut self, m: Match) {
		let summoner_champ_idx = m.get_summoner_champion_idx();
		let win_increment = match m.is_summoner_win() {
			true => 1,
			false => 0
		};
		for champ_idx in m.get_same_team_champion_idxs() {
			self.resize_if_needed(summoner_champ_idx, *champ_idx);
			self.ally_games[summoner_champ_idx][*champ_idx] += 1;
			self.ally_wins[summoner_champ_idx][*champ_idx] += win_increment;
		}
		for champ_idx in m.get_opposing_team_champion_idxs() {
			self.resize_if_needed(summoner_champ_idx, *champ_idx);
			self.vs_games[summoner_champ_idx][*champ_idx] += 1;
			self.vs_wins[summoner_champ_idx][*champ_idx] += win_increment;
		}
	}

	fn resize_if_needed(&mut self, idx1: usize, idx2: usize) {
		if idx1 >= self.ally_games.len() {
			self.ally_games.resize(idx1, Vec::new());
			self.ally_wins.resize(idx1, Vec::new());
			self.vs_games.resize(idx1, Vec::new());
			self.vs_wins.resize(idx1, Vec::new());
		}
		if idx2 >= self.ally_games[idx1].len() {
			self.ally_games[idx1].resize(idx2, 0);
			self.ally_wins[idx1].resize(idx2, 0);
			self.vs_games[idx1].resize(idx2, 0);
			self.vs_wins[idx1].resize(idx2, 0);
		}
	}

}