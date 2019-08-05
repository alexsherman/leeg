pub extern crate redis;
pub use self::redis::{Client, Connection, Commands, RedisError};
use reqs::GlobalServiceWithWeight;
extern crate serde_json;
use self::serde_json::json;
use super::summoner_utils::{Region, Masteries};
use std::env;

pub const REDIS_DEFAULT_EXPIRE_TIME: usize = 3600;
pub const REDIS_DEFAULT_EXPIRE_TIME_SUMMONER_ID: usize = 86400;
const REDIS_HOST_KEY: &str = "REDISHOST";
const REDIS_PORT_KEY: &str = "REDISPORT";

pub fn get_connection() -> Result<Connection, RedisError> {
    let ip: String = match env::var(REDIS_HOST_KEY) {
        Ok(host) => host,
        Err(_) => "127.0.0.1".to_string()
    };
    let port: String =  match env::var(REDIS_PORT_KEY) {
        Ok(p) => p,
        Err(_) => "6379".to_string()
    };
    let client_str: &str = &format!("redis://{}:{}", ip, port);
    debug!("client str for redis: {}", client_str);
    let client = redis::Client::open(client_str)?;    
    client.get_connection()
}
/**
*   E.g. team - Vec<Annie, Sivir> , opp - Vec<Vayne> -> globalreqs+Annie,Sivir-Vayne
*/
fn keyname_from_picks(team_picks: &Vec<String>, opp_picks: &Vec<String>) -> String {
    let mut tp = team_picks.clone();
    let mut op = opp_picks.clone();
    tp.sort();
    op.sort();
    format!("globalreqs+{}-{}", tp.join(","), op.join(","))
}

/**
*    E.g. sleepo mode - NA -> summonerid+sleepo mode-NA
*/
fn keyname_from_name_and_region(name: &String, region: &Region) -> String {
    format!("summonerid+{}-{}", name, region.to_string())
}

fn keyname_from_id_masteries(id: &String) -> String {
    format!("masteries+{}", id)
}

/**
* Get cached GlobalServiceWithWeight from Redis if one exists.
*/
pub fn get_cached_global_reqs(conn: &Connection, team_picks: &Vec<String>, opp_picks: &Vec<String>) 
                                -> Result<GlobalServiceWithWeight, RedisError> {
    let key = keyname_from_picks(team_picks, opp_picks);
    println!("getting reqs for {}", key);
    match get_key_from_cache(&conn, &key) {
        Ok(res) => Ok(serde_json::from_str(&(res)).unwrap()),
        Err(e) => Err(e)
    }
}

/**
* Inserts GlobalServiceWithWeight to Redis. If expire_time is None, the key will never expire on its own.
*/
pub fn insert_cached_global_reqs(conn: &Connection, team_picks: &Vec<String>, opp_picks: &Vec<String>, 
                                 service: GlobalServiceWithWeight, expire_time: Option<usize>)
                                 -> Result<Vec<String>, RedisError> {
    let key = keyname_from_picks(team_picks, opp_picks);
    println!("inserting reqs for {}", key);
    let val = json!(service).to_string();
    insert_key_value_to_cache(&conn, key, val, expire_time)
}

/**
*   Gets cached summoner id from name and region, if one exists.
*/
pub fn get_cached_summoner_id(conn: &Connection, name: &String, region: &Region) 
                              -> Result<String, RedisError> {
    let key = keyname_from_name_and_region(name, region);
    println!("getting id for {}", key);
    get_key_from_cache(&conn, &key)
}

/**
*   Inserts summoner id to redis - default expire time 1 day.
*/
pub fn insert_cached_summoner_id(conn: &Connection, name: &String, 
                                 region: &Region, id: &String) 
                                 -> Result<Vec<String>, RedisError> {
    let key = keyname_from_name_and_region(name, region);
    println!("inserting id for {}", key);
    insert_key_value_to_cache(&conn, key, id.clone(), Some(REDIS_DEFAULT_EXPIRE_TIME_SUMMONER_ID))
}

/**
*   Gets cached summoner masteries by id, if one exists.
*/
pub fn get_cached_summoner_masteries(conn: &Connection, id: &String) -> Result<Masteries, RedisError> {
    let key = keyname_from_id_masteries(id);
    println!("getting masteries for {}", key);
    match get_key_from_cache(&conn, &key) {
        Ok(res) => Ok(serde_json::from_str(&(res)).unwrap()),
        Err(e) => Err(e)
    }
}


/**
*   Inserts summoner masteries to redis - default expire time 1 day.
*/
pub fn insert_cached_summoner_masteries(conn: &Connection, id: &String, 
                                        masteries: Masteries) -> Result<Vec<String>, RedisError> {
    let key = keyname_from_id_masteries(id);
    println!("inserting masteries for {}", key);
    let val = json!(masteries).to_string();
    insert_key_value_to_cache(&conn, key, val, Some(REDIS_DEFAULT_EXPIRE_TIME_SUMMONER_ID))
}

/**
* Yep.
*/
fn get_key_from_cache(conn: &Connection, key: &String) -> Result<String, RedisError> {
    conn.get(key)
}

/**
* If an expiry time in seconds is specified, set the key to expire then. Otherwise, set without expiry.
*/
fn insert_key_value_to_cache(conn: &Connection, key: String, val: String, expire_time: Option<usize>) 
                                -> Result<Vec<String>, RedisError> {
     match expire_time {
        Some(time) => {
            conn.set_ex(key, val, time)
        },
        None => {
            conn.set(key, val)
        }
    }
}