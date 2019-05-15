#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::request::Form;
use rocket_contrib::json::JsonValue;
use std::vec::Vec;
use reqs::handle_req_req;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Banana (a Rocket server, courtesy of Prentice Pirate)"
}

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
    json!(handle_req_req(&summoner_name, &champStringToVec(&team), &champStringToVec(&opp), 
                         &champStringToVec(&tbans), &champStringToVec(&obans)))
}

fn main() {
    rocket::ignite().mount("/", routes![index, recommendation]).launch();
}
