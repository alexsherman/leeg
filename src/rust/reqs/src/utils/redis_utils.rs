pub extern crate redis;
extern crate serde;

pub use self::redis::RedisError;
use self::redis::Connection;
use self::serde::{Serialize, Deserialize};
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

pub trait Cacheable<'de> {
    type CacheItem: Deserialize<'de> + Serialize;

    fn from_cache(self, conn: &Connection) -> Result<Self::CacheItem, RedisError>;

    fn insert_into_cache(&self, conn: &Connection) -> Result<Vec<String>, RedisError>;
}