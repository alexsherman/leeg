#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::request::Form;
use rocket_contrib::json::JsonValue;
use std::vec::Vec;
use reqslib::handle_req_req;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Banana (a Rocket server, courtesy of Prentice Pirate)"
}

fn champStringToVec(championString: &String) -> Vec<String> {
    championString.split(',').map(|s| s.to_string()).collect()
}

#[get("/req?<req_num>&<team>&<opp>")]
fn recommendation(req_num: usize, team: String, opp: String) -> JsonValue {
    json!(handle_req_req(req_num, &champStringToVec(&team), &champStringToVec(&opp)))
}

fn main() {
    rocket::ignite().mount("/", routes![index, recommendation]).launch();
}
