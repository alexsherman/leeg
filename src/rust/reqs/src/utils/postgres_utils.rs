extern crate postgres;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

pub use self::postgres::{Connection, Error, TlsMode};


pub type ConnectionPool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager>;

const DB_CONFIG_PATH: &str = "Db_config.toml";
// TODO: it would be pretty cool to have a macro that takes care of this stuff depending on arrays
// you pass in or something
pub const Q_ALL_MATCHES: &str = "select blue_wins, blue_team, red_team, blue_bans, red_bans from all_matches_2";
pub const Q_GLOBAL_MATCHES_BOTH_TEAM_BLUE: &str = "select blue_wins, blue_team, red_team, blue_bans, red_bans from all_matches_2 where blue_team @> $1 and red_team @> $2";
pub const Q_GLOBAL_MATCHES_BOTH_TEAM_RED: &str = "select blue_wins, blue_team, red_team, blue_bans, red_bans from all_matches_2 where red_team @> $1 and blue_team @> $2";
pub const Q_ALL_CHAMPIONS: &str = "select * from champions order by id";
pub const Q_SUMMONER_MASTERIES: &str = "SELECT champion_id, mastery_level, mastery_points FROM summoner_masteries WHERE summoner_id = $1";
pub const INSERT_SUMMONER_MASTERIES: &str = "INSERT INTO summoner_masteries
                                        (summoner_id, champion_id, mastery_level, mastery_points)
                                        VALUES ($1, $2, $3, $4)
                                        ON CONFLICT (summoner_id, champion_id) DO UPDATE
                                        SET mastery_level = excluded.mastery_level,
                                            mastery_points = excluded.mastery_points,
                                            last_played = excluded.last_played";
/**
 ** Config toml file to connect to database.
 **/

#[derive(Deserialize)]
#[allow(dead_code)]
struct Config {
    database: String,
    host: String,
    user: String,
    password: String,
    port: String
}

/*
    Get a string representing the location of the database. Derived from db config file, but
    the host name can be overwritten by POSTGRES_CONNECTION_NAME environment variable.
*/
pub fn get_connection_string() -> String {
    let mut config_file = File::open(&Path::new(DB_CONFIG_PATH)).expect("No db config toml found!");
    let mut config_string = String::new();
    config_file.read_to_string(&mut config_string).unwrap();
    let config: Config = toml::from_str(&config_string).unwrap();   
    let host = match env::var("POSTGRES_CONNECTION_NAME") {
        Ok(connection_name) => connection_name,
        Err(_) => config.host
    };

    let connection_string = format!("postgres://{}:{}@{}/{}", config.user, config.password, host, config.database);
    connection_string
}

pub trait FromPostgres {
    type Data; 
    fn from_database(&self, pool: ConnectionPool) -> Result<Self::Data, self::postgres::Error>;
}

pub trait ToPostgres {
    fn insert_into_database(&self, pool: ConnectionPool) -> Result<(), self::postgres::Error>;
}
