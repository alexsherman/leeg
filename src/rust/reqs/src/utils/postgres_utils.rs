extern crate postgres;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use self::postgres::{Connection, Error, TlsMode};

const DB_CONFIG_PATH: &str = "Db_config.toml";

pub const Q_MOST_RECENT_ID_BY_NAME: &str = "SELECT id from summoner_matches where name = $1 ORDER BY play_date desc LIMIT 1";
pub const Q_SUMM_MATCHES_FOR_ID: &str = "SELECT * from summoner_matches where id = $1";

/**
 ** Config toml file to connect to database.
 **/

#[derive(Deserialize)]
struct Config {
    database: String,
    host: String,
    user: String,
    password: String,
    port: String
}

pub fn get_connection_to_matches_db() -> Result<Connection, Error> {
    let mut config_file = File::open(&Path::new(DB_CONFIG_PATH)).expect("No db config toml found!");
    let mut config_string = String::new();
    config_file.read_to_string(&mut config_string)?;
    let config: Config = toml::from_str(&config_string).unwrap();
    let connection_string = format!( "postgres://{}:{}@{}:{}/{}", config.user, config.password, config.host, config.port,
                                    config.database);
    Connection::connect(connection_string, TlsMode::None)
}
