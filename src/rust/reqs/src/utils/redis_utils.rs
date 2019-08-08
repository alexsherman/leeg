pub extern crate redis;
extern crate serde_json;
extern crate serde;

use self::redis::{Client, Connection, Commands};
pub use self::redis::RedisError;
use reqs::GlobalServiceWithWeight;
use self::serde::{Serialize, Deserialize};
use self::serde_json::json;
use super::summoner_utils::{Region, Masteries};
use std::env;

pub const REDIS_DEFAULT_EXPIRE_TIME: usize = 3600;
pub const REDIS_DEFAULT_EXPIRE_TIME_SUMMONER_ID: usize = 86400; 
const REDIS_HOST_KEY: &str = "REDISHOST";
const REDIS_PORT_KEY: &str = "REDISPORT";

pub type RedisConnection = Connection;

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
*    E.g. sleepo mode - NA -> summonerid+sleepo mode-NA
*/
fn keyname_from_name_and_region(name: &String, region: &Region) -> String {
    format!("summonerid+{}-{}", name, region.to_string())
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
* Yep.
*/
pub fn get_key_from_cache(conn: &Connection, key: &String) -> Result<String, RedisError> {
    conn.get(key)
}

/**
* If an expiry time in seconds is specified, set the key to expire then. Otherwise, set without expiry.
*/
pub fn insert_key_value_to_cache(conn: &Connection, key: String, val: String, expire_time: Option<usize>) 
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

pub trait Cacheable<'de> {
    type CacheItem: Deserialize<'de> + Serialize;

    fn from_cache(self, conn: &Connection) -> Result<Self::CacheItem, RedisError>;

    fn insert_into_cache(&self, conn: &Connection) -> Result<Vec<String>, RedisError>;
}