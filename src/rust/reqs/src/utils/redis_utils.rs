pub extern crate redis;
pub use self::redis::{Client, Connection, Commands, RedisError};
use reqs::GlobalServiceWithWeight;
extern crate serde_json;
use self::serde_json::json;

pub const REDIS_DEFAULT_EXPIRE_TIME: usize = 3600;

pub fn get_connection() -> Connection {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();    
    client.get_connection().unwrap()
}

/**
*   E.g. team - Vec<Annie, Sivir> , opp - Vec<Vayne> -> Annie,Sivir-Vayne
*/
fn keyname_from_picks(team_picks: &Vec<String>, opp_picks: &Vec<String>) -> String {
    let mut tp = team_picks.clone();
    let mut op = opp_picks.clone();
    tp.sort();
    op.sort();
    format!("globalreqs{}-{}", tp.join(","), op.join(","))
}

pub fn get_cached_global_reqs(conn: &Connection, team_picks: &Vec<String>, opp_picks: &Vec<String>) 
                                -> Result<GlobalServiceWithWeight, RedisError> {
    let key = keyname_from_picks(team_picks, opp_picks);
    println!("getting reqs for {}", key);
    match get_key_from_cache(&conn, &key) {
        Ok(res) => Ok(serde_json::from_str(&(res)).unwrap()),
        Err(e) => Err(e)
    }
}

pub fn insert_cached_global_reqs(conn: &Connection, team_picks: &Vec<String>, opp_picks: &Vec<String>, 
                                 service: GlobalServiceWithWeight, expire_time: Option<usize>)
                                 -> Result<Vec<String>, RedisError> {
    let key = keyname_from_picks(team_picks, opp_picks);
    println!("inserting reqs for {}", key);
    let val = json!(service).to_string();
    insert_key_value_to_cache(&conn, key, val, expire_time)
}

fn get_key_from_cache(conn: &Connection, key: &String) -> Result<String, RedisError> {
    conn.get(key)
}

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