extern crate serde_json;
extern crate serde;

use utils::postgres_utils::*;
use utils::redis_utils::{Cacheable, RedisConnection, RedisError, REDIS_DEFAULT_EXPIRE_TIME_SUMMONER_ID};
use utils::redis_utils::redis::Commands;
use utils::riot_api_utils::*;
use utils::summoner_utils::*;
use std::error::Error;
use self::serde_json::json;


#[derive(Debug, Deserialize, Serialize)]
pub struct Summoner {
    info: SummonerInfo,
    masteries: Masteries
}

impl Summoner {

    pub fn from_name_and_region(redis_conn: &RedisConnection, pool: ConnectionPool, 
                                name: String, region: Region) 
                                -> Result<Summoner, Box<Error>> {
        let info = get_summoner_info(redis_conn, name, region)?;
        let masteries = get_summoner_masteries(redis_conn, pool, &info.id)?;
        Ok(Summoner {
            info: info,
            masteries: masteries
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct SummonerInfo {
    name: String,
    id: SummonerId,
    region: Region, 
}

impl SummonerInfo {
    pub fn with_name_and_region(name: String, region: Region) -> SummonerInfo {
        SummonerInfo {
            name: name,
            region: region,
            id: "".to_string()
        }
    }

    fn get_cache_key_name(&self) -> String {
        format!("summonerid+{}-{}", self.name, self.region.to_string())
    }
}

impl Cacheable<'_> for SummonerInfo {
    type CacheItem = SummonerInfo;

    fn from_cache(self, conn: &RedisConnection) -> Result<Self::CacheItem, RedisError> {
        let key = self.get_cache_key_name();
        let result: String = conn.get(key)?;
        let id = serde_json::from_str(&(result)).unwrap();
        let mut info = SummonerInfo::with_name_and_region(self.name, self.region);
        info.id = id;
        Ok(info)
    }

    fn insert_into_cache(&self, conn: &RedisConnection) -> Result<Vec<String>, RedisError> {
        let key = self.get_cache_key_name();
        conn.set_ex(key, json!(self.id).to_string(), REDIS_DEFAULT_EXPIRE_TIME_SUMMONER_ID)
    }
}

/**
*   Check Redis for an id matching this name and region. If it doesn't exist, request from Riot API and try to cache.
*   Returns an error only if the id is not in the cache and the request to Riot API fails.
*   TODO: region included for future use. currently, all requests to Riot are going to the NA endpoint.
*/
fn get_summoner_info (conn: &RedisConnection, name: String, region: Region) -> Result<SummonerInfo, Box<Error>> {
    match SummonerInfo::with_name_and_region(name.clone(), region.clone()).from_cache(conn) {
         Ok(info) => {
            return Ok(info);
        },
        Err(_) => ()
    }
    let id = SummonerId::from_riot_api(&name.clone())?;
    let mut info = SummonerInfo::with_name_and_region(name, region);
    info.id = id;
    match info.insert_into_cache(conn) {
        Ok(_) => (),
        Err(e) => { 
            println!("{:?}", e);
        }
    }
    Ok(info)
}

/**
*   Check Redis for the summoner's masteries. If they exist, return the masteries immediately.
*   If Redis doesn't have them, check the database. If they exist, cache them and then return them.
*   If the database does not have them, then request them from Riot's API.
*   Then insert into the database and the cache and return.
*/
fn get_summoner_masteries(redis_conn: &RedisConnection, pool: ConnectionPool, id: &String) 
                          -> Result<Masteries, Box<Error>> {
    // try cache
    match  Masteries::with_id(id).from_cache(redis_conn) {
        Ok(masteries) => {
            debug!("Successfully got masteries from Redis");
            return Ok(masteries);
        },
        Err(_) => ()
    };
    match Masteries::with_id(id).from_database(pool.clone()) {
        Ok(masteries) => {
            debug!("Successfully got masteries from DB");
            match masteries.insert_into_cache(redis_conn) {
                Ok(_) => (),
                Err(e) => { 
                    debug!("{:?}", e);
                }
            };
            return Ok(masteries);
        },
        Err(_) => ()
    };
    // get from api, then try insert to db, then insert into cache, and return
    let masteries = Masteries::from_riot_api(id)?;
    println!("Successfully got masteries from API, inserting into DB and Redis");
    match masteries.insert_into_database(pool) {
        Ok(_) => {
            match masteries.insert_into_cache(redis_conn) {
                Ok(_) => (),
                Err(e) => { 
                    debug!("redis {:?}", e);
                }
            };
        },
        Err(e) => { 
            println!("postgres {:?}", e);
            match masteries.insert_into_cache(redis_conn) {
                Ok(_) => (),
                Err(er) => { 
                    debug!("redis {:?}", er);
                }
            };
        }
    }
    Ok(masteries)
}