extern crate serde_json;

use utils::postgres_utils::*;
use utils::redis_utils::{RedisConnection, Cacheable, RedisError, REDIS_DEFAULT_EXPIRE_TIME};
use self::serde_json::json;
use utils::redis_utils::redis::Commands;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Deserialize, Serialize, Clone)] 
pub enum Region {
    BR,
    EUNE,
    EUW,
    JP,
    KR,
    LAN,
    LAS,
    NA,
    OCE,
    TR,
    RU,
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Mastery {
    #[serde(rename="championId")]
    pub champion_id: i16,
    #[serde(rename="championLevel")]
    pub mastery_level: i16,
    #[serde(rename="championPoints")]
    pub mastery_points: i32
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Masteries {
    pub id: String,
    pub map: HashMap<i16, Mastery>
}

impl Masteries {

    pub fn populate_from_mastery_vec(mut self, response: Vec<Mastery>) -> Masteries {
        for mastery in response {
            self.map.insert(mastery.champion_id, mastery);
        }
        self
    }

    pub fn with_id(id: &String) -> Masteries {
        Masteries {
            id: id.clone(),
            map: HashMap::new()
        }
    }

    fn get_cache_key_name(&self) -> String {
        format!("masteries+{}", self.id)
    }

}

impl FromDatabase for Masteries {
    type Data = Masteries; 

    fn from_database(self, pool: ConnectionPool) -> Result<Masteries, Error> {
        let conn = pool.get().unwrap();
        let mut mastery_vec: Vec<Mastery> = Vec::new();
        for row in &conn.query(Q_SUMMONER_MASTERIES, &[&self.id])? {
            mastery_vec.push(Mastery {
                champion_id: row.get(0),
                mastery_level: row.get(1),
                mastery_points: row.get(2)
            });
        }
        Ok(self.populate_from_mastery_vec(mastery_vec))
    }
}

impl ToDatabase for Masteries {
    fn insert_into_database(&self, pool: ConnectionPool) -> Result<(), Error> {
        let conn = pool.get().unwrap();
        let transaction = conn.transaction()?;
        let stmt = transaction.prepare(INSERT_SUMMONER_MASTERIES)?;
        for (_, mastery) in self.map.clone().into_iter() {
            stmt.execute(&[&self.id, &mastery.champion_id, &mastery.mastery_level, &mastery.mastery_points])?;
        }
        transaction.commit()?;
        Ok(())
    }
}

impl Cacheable<'_> for Masteries {
    type CacheItem = Masteries;

    fn from_cache(self, conn: &RedisConnection) -> Result<Masteries, RedisError> {
        debug!("getting masteries for {}", self.id);

        let key = self.get_cache_key_name();
        let result: String = conn.get(key)?;
        Ok(serde_json::from_str(&(result)).unwrap())
    }

    fn insert_into_cache(&self, conn: &RedisConnection) -> Result<Vec<String>, RedisError> {
         debug!("inserting masteries for {}", self.id);
       
        let key = self.get_cache_key_name();
        conn.set_ex(key, json!(self).to_string(), REDIS_DEFAULT_EXPIRE_TIME)
    }
}