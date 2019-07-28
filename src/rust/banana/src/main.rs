#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate r2d2;
extern crate postgres;
extern crate r2d2_postgres;

use rocket_contrib::json::JsonValue;
use std::vec::Vec;
use reqs::{handle_global_req_req, load_champions_from_db,  get_summoner_mastery_by_name, Champions, get_connection_string};
use rocket::{Request, Response, State};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, ContentType, Method};
use std::io::Cursor;
use r2d2_postgres::{PostgresConnectionManager,TlsMode};

fn champ_string_to_vec(champ_string: &Option<String>) -> Vec<String> {
    match champ_string {
        Some(s) => {
           s.split(',').map(|s| s.to_string()).collect() 
        },
        None => {
            Vec::new()
        }
    }
}

#[get("/globalreq?<team>&<opp>&<roles>")]
fn global_recommendation(team: Option<String>, opp: Option<String>, roles: Option<String>, 
                         champions: State<Champions>, postgres_pool: State<r2d2::Pool<PostgresConnectionManager>>) 
                         -> JsonValue {
    let roles_option = match roles {
        Some(s) => {
           Some(s.split(',').map(|s| s.to_string()).collect()) 
        },
        None => None
    };
    json!({"reqs": handle_global_req_req(
                    &champ_string_to_vec(&team), 
                    &champ_string_to_vec(&opp), 
                    roles_option, 
                    &champions, 
                    postgres_pool.clone()
                   )
        })
}
/*
#[get("/championmatrix")]
fn champion_matrix() -> JsonValue {
    json!(get_global_matrix())
}
*/ 

#[get("/summonermasteries?<name>")]
fn summoner_masteries(name: String, postgres_pool: State<r2d2::Pool<PostgresConnectionManager>>) -> JsonValue {
    match get_summoner_mastery_by_name(name, postgres_pool.clone()) {
        Ok(masteries) => json!(masteries),
        Err(_) => json!("no data")
    }
} 

#[get("/champions")]
fn champions(champions: State<Champions>) -> JsonValue {
    json!(champions.get_list().clone())
}

fn main() {
    
    let champions: Champions = load_champions_from_db().unwrap();
    let manager: PostgresConnectionManager = PostgresConnectionManager::new(get_connection_string(), TlsMode::None).unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();
    // this will put all global winrates and 1 to 1 winrate services in cache if not cached already
    handle_global_req_req(&Vec::new(), &Vec::new(), None, &champions, pool.clone());
    rocket::ignite()
        .manage(champions)
        .manage(pool)
        .attach(CORS())
        .mount("/", routes![global_recommendation, summoner_masteries, champions])
        .launch();
}

pub struct CORS();

//https://github.com/SergioBenitez/Rocket/issues/25
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}