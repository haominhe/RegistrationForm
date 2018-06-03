#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{Json, Value};
use rocket::response::{self, Response, Responder};

// #[get("/<name>/<age>")]
// fn hello(name: String, age: u8) -> String {
//     format!("Hello, {} year old named {}!", age, name)
// }
mod family;
use family::{Family};

#[post("/", data = "<family>")]
fn create(family: Json<Family>) -> Json<Family> {
     family
}

#[options("/")]
fn options_handler<'a>() -> Response<'a> {
    Response::build()
        .finalize()
}

// #[get("/registered")]
// fn get_registered() -> 



fn main() {
    rocket::ignite()
        // .mount("/hello", routes![hello])
        .mount("/register", routes![create, options_handler])
        // .mount("/heroes", routes![read])
        .launch();
}