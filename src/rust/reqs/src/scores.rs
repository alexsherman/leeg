/**
 * Structs and traits for representing win-rate matrices
 * @author dmcfalls
 * @author alexsherman
 */

use matches::GlobalMatch;

/**
 * Represents a score indicating how well one champion fares against another.
 * The "meaning" of a Score is determined by the structure using it,
 * but a higher score is generally better than a lower one.
 */
pub type Score = f64;

pub struct GlobalMatchCounts {
    same_wins: Vec<u32>,
    same_games: Vec<u32>,
    opp_wins: Vec<u32>,
    opp_games: Vec<u32>,
    total_games: u32,
    same_bans: Vec<u32>,
    opp_bans: Vec<u32>
}

impl GlobalMatchCounts {
    fn with_dimensions(n: usize) -> GlobalMatchCounts {
        GlobalMatchCounts {
            same_wins: vec![0u32; n],
            same_games: vec![0u32; n],
            opp_wins: vec![0u32; n],
            opp_games: vec![0u32; n],
            total_games: 0,
            same_bans: vec![0u32; n],
            opp_bans: vec![0u32; n]
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
        for champ_idx in m.get_same_bans() {
            self.same_bans[*champ_idx] += 1;
        }
        for champ_idx in m.get_opp_bans() {
            self.opp_bans[*champ_idx] += 1;
        }
        self.total_games += 1;
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct GlobalScoreVectors {
    pub same_winrates: Vec<Score>,
    pub opp_winrates: Vec<Score>,
    pub same_pickrates: Vec<Score>,
    pub opp_pickrates: Vec<Score>,
    pub same_banrates: Vec<Score>,
    pub opp_banrates: Vec<Score>,
    pub n: usize
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
            same_winrates: vec![0f64; n],
            opp_winrates: vec![0f64; n],
            same_pickrates: vec![0f64; n],
            opp_pickrates: vec![0f64; n],
            same_banrates: vec![0f64; n],
            opp_banrates: vec![0f64; n],
			n: n
		}
	}

    fn get_scores_from_match_counts(&mut self, match_counts: &GlobalMatchCounts) {
        for i in 0..self.n {
            self.same_winrates[i] = self.winrate_score(match_counts.same_wins[i], match_counts.same_games[i]);
            self.opp_winrates[i] = self.winrate_score(match_counts.opp_wins[i], match_counts.opp_games[i]);
            self.same_pickrates[i] = self.calc_pickrate(match_counts.same_games[i], match_counts.total_games);
            self.opp_pickrates[i] = self.calc_pickrate(match_counts.opp_games[i], match_counts.total_games);
            self.same_banrates[i] = self.calc_pickrate(match_counts.same_bans[i], match_counts.total_games);
            self.opp_banrates[i] = self.calc_pickrate(match_counts.opp_bans[i], match_counts.total_games);
        }
    }
    
	fn winrate_score(&self, wins: u32, games: u32) -> f64 {
		if games == 0u32 {
			return 1f64;
		}
        let raw_winrate = wins as f64 / games as f64;
        let mut small_sample_penalty = 0f64;
        let mut factor = 0f64;
        if games < 100 {
            small_sample_penalty = 0.5 as f64 / games as f64;
        }
        if raw_winrate > 0.6 {
            factor = 1f64;
        } else if raw_winrate < 0.4 {
            factor = -1f64;
        }
      //  println!("{} - {} = {}", raw_winrate, small_sample_penalty, raw_winrate - small_sample_penalty);
		raw_winrate - small_sample_penalty * factor
	}

    fn calc_pickrate(&self, games: u32, total_games: u32) -> f64 {
        match total_games {
            0u32 => 0f64,
            _ => games as f64 / total_games as f64
        }
    }
}
