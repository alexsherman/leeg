use utils::postgres_utils::*;
use utils::redis_utils::{RedisConnection, get_cached_summoner_id, get_cached_summoner_masteries, insert_cached_summoner_id, insert_cached_summoner_masteries};
use utils::riot_api_utils::*;
use utils::summoner_utils::*;
use std::error::Error;


#[derive(Debug, Deserialize, Serialize)]
pub struct Summoner {
    info: SummonerInfo,
    masteries: Masteries
}

#[derive(Debug, Deserialize, Serialize)]
struct SummonerInfo {
    name: String,
    id: SummonerId,
    region: Region, 
}

impl Summoner {

    pub fn from_name_and_region(redis_conn: &RedisConnection, pool: ConnectionPool, 
                                name: String, region: Region) 
                                -> Result<Summoner, Box<Error>> {
        let id = get_summoner_id(redis_conn, &name, &region)?;
        let masteries = get_summoner_masteries(redis_conn, pool, &id)?;
        let summoner_info = SummonerInfo {
            name: name,
            id: id,
            region: region
        };
        Ok(Summoner {
            info: summoner_info,
            masteries: masteries
        })
    }
}

/**
*   Check Redis for an id matching this name and region. If it doesn't exist, request from Riot API and try to cache.
*   Returns an error only if the id is not in the cache and the request to Riot API fails.
*   TODO: region included for future use. currently, all requests to Riot are going to the NA endpoint.
*/
fn get_summoner_id (conn: &RedisConnection, name: &String, region: &Region) -> Result<String, Box<Error>> {
    match get_cached_summoner_id(conn, name, region) {
        Ok(id) => {
            return Ok(id);
        },
        Err(_) => ()
    };
    let id = SummonerId::from_riot_api(name)?;
    match insert_cached_summoner_id(conn, name, region, &id) {
        Ok(_) => (),
        Err(e) => { 
            println!("{:?}", e);
        }
    };
    Ok(id)
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
    match get_cached_summoner_masteries(redis_conn, id) {
        Ok(masteries) => {
            println!("Successfully got masteries from Redis");
            return Ok(masteries);
        },
        Err(_) => ()
    };
    match Masteries::with_id(id).from_database(pool.clone()) {
        Ok(masteries) => {
            println!("Successfully got masteries from DB");
            match insert_cached_summoner_masteries(redis_conn, id, masteries.clone()) {
                Ok(_) => (),
                Err(e) => { 
                    println!("{:?}", e);
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
            match insert_cached_summoner_masteries(redis_conn, id, masteries.clone()) {
                Ok(_) => (),
                Err(e) => { 
                    println!("redis {:?}", e);
                }
            };
        },
        Err(e) => { 
            println!("postgres {:?}", e);
            match insert_cached_summoner_masteries(redis_conn, id, masteries.clone()) {
                Ok(_) => (),
                Err(er) => { 
                    println!("redis {:?}", er);
                }
            };
        }
    }
    Ok(masteries)
}