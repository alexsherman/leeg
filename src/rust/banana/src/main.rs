#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::json::JsonValue;
use std::vec::Vec;
use reqs::{handle_req_req, handle_global_req_req, get_global_matrix, load_champions_from_db, Champions};
use rocket::{Request, Response, State};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, ContentType, Method};
use std::io::Cursor;

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

#[get("/req?<summoner_name>&<team>&<opp>&<tbans>&<obans>")]
fn recommendation(summoner_name: String, team: Option<String>, opp: Option<String>, 
                  tbans: Option<String>, obans: Option<String>, champions: State<Champions>) 
                  -> JsonValue {
    json!({"reqs": handle_req_req(&summoner_name, &champ_string_to_vec(&team), &champ_string_to_vec(&opp), 
                         &champ_string_to_vec(&tbans), &champ_string_to_vec(&obans), &champions)})
}

#[get("/globalreq?<team>&<opp>&<roles>")]
fn global_recommendation(team: Option<String>, opp: Option<String>, roles: Option<String>, champions: State<Champions>) -> JsonValue {
    let roles_option = match roles {
        Some(s) => {
           Some(s.split(',').map(|s| s.to_string()).collect()) 
        },
        None => None
    };
    json!({"reqs": handle_global_req_req(&champ_string_to_vec(&team), &champ_string_to_vec(&opp), roles_option, &champions)})
}

#[get("/championmatrix")]
fn champion_matrix() -> JsonValue {
    json!(get_global_matrix())
}

fn main() {
    // this will put all global winrates and 1 to 1 winrate services in cache if not cached already
    let champions: Champions = load_champions_from_db().unwrap();
    handle_global_req_req(&Vec::new(), &Vec::new(), None, &champions);
    rocket::ignite().manage(champions).attach(CORS()).mount("/", routes![recommendation, global_recommendation, champion_matrix]).launch();
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