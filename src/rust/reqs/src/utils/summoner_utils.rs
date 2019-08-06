use postgres_utils::*;
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

    pub fn from_mastery_vec(id: &String, response: Vec<Mastery>) -> Masteries {
        let mut masteries = Masteries {
            id: id.clone(),
            map: HashMap::new()
        };
        for mastery in response {
            masteries.map.insert(mastery.champion_id, mastery);
        }
        masteries
    }

}

impl FromPostgres for Masteries {
    type Data = Masteries; 
    fn from_database(&self, pool: ConnectionPool) -> Result<Self::Data, self::postgres::Error> {
        unimplemented!();
    };
}