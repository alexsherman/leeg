extern crate reqwest;
extern crate postgres;

use std::error::Error;
use std::env;
use summoner_utils::{Mastery, Masteries};

const RIOT_API_URL_ROOT: &str = "https://na1.api.riotgames.com";
const RIOT_API_SUMMONER_PATH: &str = "/lol/summoner/v4/summoners/by-name/";
const RIOT_API_MASTERIES_PATH: &str = "/lol/champion-mastery/v4/champion-masteries/by-summoner/";
const RIOT_API_KEY_PARAM: &str = "?api_key=";
const RIOT_API_KEY_ENV_VAR: &str = "RIOT_API_KEY";

#[derive(Debug, Deserialize)]
struct SummonerID {
    name: String,
    id: String
}

pub fn request_summoner_id_from_api(name: &String) -> Result<String, Box<Error>> {
    let riot_api_key = env::var(RIOT_API_KEY_ENV_VAR)?;
    let query_url = format!("{}{}{}{}{}", RIOT_API_URL_ROOT, 
                             RIOT_API_SUMMONER_PATH, name, 
                             RIOT_API_KEY_PARAM, riot_api_key
                            );
    let id: String = reqwest::get(&query_url)?.json()?.id;
    Ok(id)
}


pub fn request_masteries_from_api(id: &String) -> Result<Masteries, Box<Error>> {
    let riot_api_key = env::var(RIOT_API_KEY_ENV_VAR)?;
    let query_url = format!("{}{}{}{}{}", RIOT_API_URL_ROOT, 
                             RIOT_API_MASTERIES_PATH, id, 
                             RIOT_API_KEY_PARAM, riot_api_key
                            );
    let response: Vec<Mastery> = reqwest::get(&query_url)?.json()?;
    let masteries = Masteries::from_mastery_response(id, response);
    Ok(masteries)
}
