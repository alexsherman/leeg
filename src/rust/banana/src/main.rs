#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::request::Form;
use rocket_contrib::json::JsonValue;
use std::vec::Vec;
use reqs::{handle_req_req, handle_global_req_req};
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, ContentType, Method};
use std::io::Cursor;

fn champStringToVec(championString: &Option<String>) -> Vec<String> {
    match championString {
        Some(s) => {
           s.split(',').map(|s| s.to_string()).collect() 
        },
        None => {
            Vec::new()
        }
    }
}

#[get("/req?<summoner_name>&<team>&<opp>&<tbans>&<obans>")]
fn recommendation(summoner_name: String, team: Option<String>, opp: Option<String>, tbans: Option<String>, obans: Option<String>) -> JsonValue {
    json!({"reqs": handle_req_req(&summoner_name, &champStringToVec(&team), &champStringToVec(&opp), 
                         &champStringToVec(&tbans), &champStringToVec(&obans))})
}

#[get("/globalreq?<team>&<opp>")]
fn global_recommendation(team: Option<String>, opp: Option<String>) -> JsonValue {
    json!({"reqs": handle_global_req_req(&champStringToVec(&team), &champStringToVec(&opp))})
}

fn main() {
    rocket::ignite().attach(CORS()).mount("/", routes![recommendation, global_recommendation]).launch();
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