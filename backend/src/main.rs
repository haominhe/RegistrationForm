#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use serde_json::Error;
use rocket_contrib::{Json, Value};
use rocket::response::{self, Response, Responder};


// References:
// https://github.com/serde-rs/json/issues/377


mod family;
use family::{Family};

static mut fam: Json =  Json(json!({"status": "ok"}));

#[post("/", data = "<family>")]
fn create(family: Json<Family>) -> Json<Family> {    
    family
}

#[get("/")]
fn read() -> Json<Value> {
    // let resp = fam.clone();
    // resp
        Json(json!(fam))

}

#[options("/")]
fn options_handler<'a>() -> Response<'a> {
    Response::build()
        .finalize()
}


fn main() {
    rocket::ignite()
        // .mount("/hello", routes![hello])
        .mount("/register", routes![create, options_handler])
        // .mount("/heroes", routes![read])
        .launch();
}