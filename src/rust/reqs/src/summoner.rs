use utils::postgres_utils;
use utils::redis_utils;
use utils::redis_utils::{get_cached_summoner_id, get_cached_summoner_masteries, insert_cached_summoner_id, insert_cached_summoner_masteries};
use utils::riot_api_utils::*;
use utils::summoner_utils::*;
use std::error::Error;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

#[derive(Debug, Deserialize, Serialize)]
pub struct Summoner {
    name: String,
    id: String,
    region: Region, 
    masteries: Masteries
}

impl Summoner {

    pub fn from_name_and_region(redis_conn: &redis_utils::Connection, pool: Pool<PostgresConnectionManager>, 
                                name: String, region: Region) 
                                -> Result<Summoner, Box<Error>> {
        let id = get_summoner_id(redis_conn, &name, &region)?;
        let masteries = get_summoner_masteries(redis_conn, pool, &id)?;
        Ok(Summoner {
            name: name,
            region: region,
            id: id,
            masteries: masteries
        })
    }
}

/**
*   Check Redis for an id matching this name and region. If it doesn't exist, request from Riot API and try to cache.
*   Returns an error only if the id is not in the cache and the request to Riot API fails.
*   TODO: region included for future use. currently, all requests to Riot are going to the NA endpoint.
*/
fn get_summoner_id (conn: &redis_utils::Connection, name: &String, region: &Region) -> Result<String, Box<Error>> {
    match get_cached_summoner_id(conn, name, region) {
        Ok(id) => {
            return Ok(id);
        },
        Err(_) => ()
    };
    let id = request_summoner_id_from_api(name)?;
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
fn get_summoner_masteries(redis_conn: &redis_utils::Connection, pool: Pool<PostgresConnectionManager>, id: &String) 
                          -> Result<Masteries, Box<Error>> {
    // try cache
    match get_cached_summoner_masteries(redis_conn, id) {
        Ok(masteries) => {
            println!("Successfully got masteries from Redis");
            return Ok(masteries);
        },
        Err(_) => ()
    };
    // try db, cache if found
    let db_conn = pool.get().unwrap();
    match get_masteries_from_database(&db_conn, id) {
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
    let masteries = request_masteries_from_api(id)?;
    println!("Successfully got masteries from API, inserting into DB and Redis");
    match insert_masteries_into_database(&db_conn, id, &masteries) {
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

fn get_masteries_from_database(conn: &postgres_utils::Connection, id: &String) -> Result<Masteries, Box<Error>> {
    let mut mastery_vec: Vec<Mastery> = Vec::new();
    for row in &conn.query(postgres_utils::Q_SUMMONER_MASTERIES, &[&id])? {
        mastery_vec.push(Mastery {
            champion_id: row.get(0),
            mastery_level: row.get(1),
            mastery_points: row.get(2)
        });
    }
    if mastery_vec.len() == 0 {
        bail!("No masteries found in DB");
    }
    Ok(Masteries::from_mastery_vec(id, mastery_vec))
}

fn insert_masteries_into_database(conn: &postgres_utils::Connection, id: &String, masteries: &Masteries) -> Result<(), Box<Error>> {
    let transaction = conn.transaction()?;
    let stmt = transaction.prepare(postgres_utils::INSERT_SUMMONER_MASTERIES)?;
    for (_, mastery) in masteries.map.clone().into_iter() {
        stmt.execute(&[id, &mastery.champion_id, &mastery.mastery_level, &mastery.mastery_points])?;
    }
    transaction.commit()?;
    Ok(())
}