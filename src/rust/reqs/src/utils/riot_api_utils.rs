extern crate reqwest;
extern crate postgres;

use std::error::Error;
use std::env;
use super::summoner_utils::{Mastery, Masteries};

const RIOT_API_URL_ROOT: &str = "https://na1.api.riotgames.com";
const RIOT_API_SUMMONER_PATH: &str = "/lol/summoner/v4/summoners/by-name/";
const RIOT_API_MASTERIES_PATH: &str = "/lol/champion-mastery/v4/champion-masteries/by-summoner/";
const RIOT_API_KEY_PARAM: &str = "?api_key=";
const RIOT_API_KEY_ENV_VAR: &str = "RIOT_API_KEY";

pub type SummonerId = String;

#[derive(Debug, Deserialize)]
struct SummonerIdContainer {
    id: SummonerId
}

pub trait FromRiotApi {
    type DeserializedResponse;
    fn from_riot_api(identifier: &str) -> Result<Self::DeserializedResponse, Box<Error>>;
}

impl FromRiotApi for SummonerId {
    type DeserializedResponse = SummonerId;

    fn from_riot_api(identifier: &str) -> Result<Self::DeserializedResponse, Box<Error>> {
        let riot_api_key = env::var(RIOT_API_KEY_ENV_VAR)?;
        let query_url = format!("{}{}{}{}{}", RIOT_API_URL_ROOT, 
                                 RIOT_API_SUMMONER_PATH, identifier, 
                                 RIOT_API_KEY_PARAM, riot_api_key
                                );
        let idc: SummonerIdContainer = reqwest::get(&query_url)?.json()?;
        Ok(idc.id)
    }
}

impl FromRiotApi for Masteries {
    type DeserializedResponse = Masteries;

    fn from_riot_api(identifier: &str) -> Result<Self::DeserializedResponse , Box<Error>> {
        let riot_api_key = env::var(RIOT_API_KEY_ENV_VAR)?;
        let query_url = format!("{}{}{}{}{}", RIOT_API_URL_ROOT, 
                                 RIOT_API_MASTERIES_PATH, identifier, 
                                 RIOT_API_KEY_PARAM, riot_api_key
                                );
        let response: Vec<Mastery> = reqwest::get(&query_url)?.json()?;
        Ok(Masteries::with_id(&identifier.to_string()).populate_from_mastery_vec(response))
    }
}
