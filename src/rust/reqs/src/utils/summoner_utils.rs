use std::collections::HashMap;
extern crate serde_json;
use self::serde_json::json;

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

#[derive(Debug, Deserialize)]
pub struct Mastery {
    #[serde(rename="championId")]
    pub champion_id: i16,
    #[serde(rename="championLevel")]
    pub mastery_level: i16,
    #[serde(rename="championPoints")]
    pub mastery_points: i32
}

#[derive(Debug, Deserialize)]
pub struct Masteries {
    pub id: String,
    pub map: HashMap<i16, Mastery>
}

impl Masteries {

    pub fn from_mastery_response(id: &String, response: Vec<Mastery>) -> Masteries {
        let masteries = Masteries {
            id: id.clone(),
            map: HashMap::new()
        };
        for mastery in response {
            masteries.insert(mastery.champion_id, mastery);
        }
    }
}