pub extern crate redis;
pub use self::redis::{Client, Connection, Commands, RedisError};
use reqs::GlobalServiceWithWeight;
extern crate serde_json;
use self::serde_json::json;

const default_expire_time: usize: 3600;

pub fn get_connection() -> Connection {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();    
    let conn = client.get_connection().unwrap();

    //testing 
    let _: () = conn.set("answer", 42).unwrap();
    let answer: usize = conn.get("answer").unwrap();
    println!("Answer: {}", answer);
    conn
}

/**
*   E.g. team - Vec<Annie, Sivir> , opp - Vec<Vayne> -> Annie,Sivir-Vayne
*/
fn keyname_from_picks(team_picks: &Vec<String>, opp_picks: &Vec<String>) -> String {
    format!("{}-{}", team_picks.join(","), opp_picks.join(","))
}

pub fn get_cached_global_reqs(conn: &Connection, team_picks: &Vec<String>, opp_picks: &Vec<String>) 
                                -> Result<GlobalServiceWithWeight, RedisError> {
    let key = keyname_from_picks(team_picks, opp_picks);
    println!("getting reqs for {}", key);
    let res: Result<String, RedisError> = conn.get(key);
    match res {
        Ok(_) => Ok(serde_json::from_str(&(res.unwrap())).unwrap()),
        Err(e) => Err(e)
    }
}

pub fn insert_cached_global_reqs(conn: &Connection, team_picks: &Vec<String>, opp_picks: &Vec<String>, service: GlobalServiceWithWeight) 
                                -> Result<Vec<String>, RedisError> {
    let key = keyname_from_picks(team_picks, opp_picks);
    println!("inserting reqs for {}", key);
    let val = json!(service).to_string();
    conn.set_ex(key, val, default_expire_time)
}