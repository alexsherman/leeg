/**
 * Contains the main workhorse functions for generating champion recommendations
 * @author dmcfalls
 */

use champions::Champion;
use champions::Champions;
use matches::Match;
use winrates::WinRates;

/**
 * A champion recommendation service. Must implement a method that, given the player's team's
 * picks, the picks on the opposing team, and num_reqs, returns num_reqs champions as
 * suggested picks for that player.
 */
trait ReqService {

	fn req(&self, team_picks: Vec<Champion>, other_picks: Vec<Champion>, num_reqs: usize)
			-> Vec<Champion>;

}

 /**
  * Champion recommendation service for a single summoner using that summoner's match history
  */
pub struct SingleSummonerReqService {
	champions: Champions,
	winrates: WinRates
}

impl SingleSummonerReqService {

	pub fn from_matches(matches: Vec<Match>, champions: &Champions) -> SingleSummonerReqService {
		// TODO: implement
		return SingleSummonerReqService { champions: champions.clone(), winrates: WinRates::new() };
	}

}

impl ReqService for SingleSummonerReqService {

	fn req(&self, team_picks: Vec<Champion>, other_picks: Vec<Champion>, num_reqs: usize)
			-> Vec<Champion> {
		// TODO: implement
		return Vec::new();
	}

}