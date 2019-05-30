/**
 * Structs and traits for representing win-rate matrices
 * @author dmcfalls
 */

use matches::{Match, GlobalMatch};

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
  * > ally_scores_2d[i][j] is the score of the summoner on 'i' playing on a team with 'j'
  * > opp_scores_2d[i][j] is the score of the summoner on 'i' in a match against 'j'
  * > champ_winrates[i] is the overall winrate of the summoner on 'i'
  */
#[derive(Debug)]  
pub struct SimpleIndependentScoreMatrix {
	ally_scores_2d: Vec<Vec<Score>>,
	opp_scores_2d: Vec<Vec<Score>>,
	champ_winrates: Vec<Score>,
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

	pub fn champ_winrate(&self, champ_idx: &usize) -> f64 {
		return self.champ_winrates[*champ_idx];
	}

	fn with_dimensions(n: usize) -> SimpleIndependentScoreMatrix {
		let mut score_matrix = SimpleIndependentScoreMatrix {
			ally_scores_2d: Vec::with_capacity(n),
			opp_scores_2d: Vec::with_capacity(n),
			champ_winrates: vec![0f64; n],
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
			self.champ_winrates[i]
					= self.winrate_score(match_counts.champ_wins[i], match_counts.champ_games[i]);
			for j in 0..self.n {
				self.ally_scores_2d[i][j]
						= self.winrate_score(match_counts.ally_wins[i][j], match_counts.ally_games[i][j]);
				self.opp_scores_2d[i][j]
						= self.winrate_score(match_counts.vs_wins[i][j], match_counts.vs_games[i][j]);
			}
		}
	}

	fn winrate_score(&self, wins: u32, games: u32) -> f64 {
		if games == 0u32 {
			return 1f64;
		}
		return wins as f64 / games as f64;
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
	vs_wins: Vec<Vec<u32>>,
	champ_games: Vec<u32>,
	champ_wins: Vec<u32>
}

impl MatchCounts {

	fn with_dimensions(n: usize) -> MatchCounts {
		let mut match_counts = MatchCounts {
			ally_games: Vec::with_capacity(n),
			ally_wins: Vec::with_capacity(n),
			vs_games: Vec::with_capacity(n),
			vs_wins: Vec::with_capacity(n),
			champ_games: vec![0u32; n],
			champ_wins: vec![0u32; n]
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
		self.champ_games[summoner_champ_idx] += 1;
		self.champ_wins[summoner_champ_idx] += win_increment;
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

pub struct GlobalMatchCounts {
    same_wins: Vec<u32>,
    same_games: Vec<u32>,
    opp_wins: Vec<u32>,
    opp_games: Vec<u32>,
    total_games: u32
}

impl GlobalMatchCounts {
    fn with_dimensions(n: usize) -> GlobalMatchCounts {
        GlobalMatchCounts {
            same_wins: vec![0u32; n],
            same_games: vec![0u32; n],
            opp_wins: vec![0u32; n],
            opp_games: vec![0u32; n],
            total_games: 0
        }
    }

    fn populate_global_match_data(&mut self, m: &GlobalMatch) {
        let win_increment = match m.same_wins {
            true => 1,
            false => 0
        };
        for champ_idx in m.get_same_team_champion_idxs() {
            self.same_games[*champ_idx] += 1;
            self.same_wins[*champ_idx] += win_increment;
        }
        for champ_idx in m.get_opp_team_champion_idxs() {
            self.opp_games[*champ_idx] += 1;
            self.opp_wins[*champ_idx] += 1 - win_increment;
        }
        self.total_games += 1;
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct GlobalScoreVectors {
    same_team: Vec<usize>,
    opp_team: Vec<usize>,
    pub same_winrates: Vec<Score>,
    pub opp_winrates: Vec<Score>,
    pub same_pickrates: Vec<Score>,
    pub opp_pickrates: Vec<Score>,
    n: usize
}

pub trait ScoreVector {
    fn from_global_matches(matches: &Vec<GlobalMatch>, n: usize) -> Self;
}

impl ScoreVector for GlobalScoreVectors {
    fn from_global_matches(matches: &Vec<GlobalMatch>, n: usize) -> GlobalScoreVectors { 
	    let mut score_vectors = GlobalScoreVectors::with_dimensions(n);
	    let mut match_counts = GlobalMatchCounts::with_dimensions(n);
	    for m in matches {
	    	match_counts.populate_global_match_data(m);
	    }
	    score_vectors.get_scores_from_match_counts(&match_counts);
	    score_vectors
    }
}
    
impl GlobalScoreVectors {
	pub fn with_dimensions(n: usize) -> GlobalScoreVectors {
        GlobalScoreVectors {
            same_team: Vec::with_capacity(5),
            opp_team: Vec::with_capacity(5),
            same_winrates: vec![0f64; n],
            opp_winrates: vec![0f64; n],
            same_pickrates: vec![0f64; n],
            opp_pickrates: vec![0f64; n],
			n: n
		}
	}

    fn get_scores_from_match_counts(&mut self, match_counts: &GlobalMatchCounts) {
        for i in 0..self.n {
            self.same_winrates[i] = self.winrate_score(match_counts.same_wins[i], match_counts.same_games[i]);
            self.opp_winrates[i] = self.winrate_score(match_counts.opp_wins[i], match_counts.opp_games[i]);
            self.same_pickrates[i] = self.calc_pickrate(match_counts.same_games[i], match_counts.total_games);
            self.opp_pickrates[i] = self.calc_pickrate(match_counts.opp_games[i], match_counts.total_games);
        }
    }
    
	fn winrate_score(&self, wins: u32, games: u32) -> f64 {
		if games == 0u32 {
			return 1f64;
		}
        let mut raw_winrate = wins as f64 / games as f64;
        let mut small_sample_penalty = 0f64;
        if games < 100 && raw_winrate > 0.55f64 {
            small_sample_penalty = 0.5 as f64 / games as f64;
        }
        println!("{} - {} = {}", raw_winrate, small_sample_penalty, raw_winrate - small_sample_penalty);
		raw_winrate - small_sample_penalty;
	}

    fn calc_pickrate(&self, games: u32, total_games: u32) -> f64 {
        games as f64 / total_games as f64
    }
}
