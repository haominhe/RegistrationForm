// References:
// https://github.com/diesel-rs/diesel/tree/master/diesel_cli
// https://github.com/rust-lang-nursery/rustup.rs
// https://github.com/diesel-rs/diesel/issues/1700
// https://github.com/rust-lang/rust/issues/50825
// https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8
// https://github.com/sean3z/rocket-diesel-rest-api-example
// https://github.com/diesel-rs/diesel/issues/321
// http://www.goldsborough.me/rust/web/tutorial/2018/01/20/17-01-11-writing_a_microservice_in_rust/
// https://github.com/SergioBenitez/Rocket/tree/v0.3.12/examples/json
// https://api.rocket.rs/rocket_contrib/enum.Value.html
// https://github.com/pwfantasy/pwfantasy-api
// https://github.com/blackbeam/rust-mysql-simple


// You may need to run the following commands in order to get this program to run: 
// sudo apt-cache search mysqlclient
// sudo apt-get install libmysqlclient-dev
// rustup run nightly-2018-05-14 bash
// rustup run nightly-2018-05-14 cargo install diesel_cli --no-default-features --features mysql


#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;


#[macro_use] extern crate serde_json;

use serde_json::Error;
use rocket_contrib::{Json, Value};
use rocket::response::{self, Response, Responder};

#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

mod db;
mod schema;

mod person;
use person::Person;


#[post("/", data = "<person>")]
fn create(person: Json<Person>, connection: db::Connection) -> Json<Person> {    
    let insert = Person { ..person.into_inner() };
    Json(Person::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> Json<Value> {
    Json(json!(Person::read(&connection)))
}

#[options("/")]
fn options_handler<'a>() -> Response<'a> {
    Response::build()
        .finalize()
}


fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/register", routes![create, options_handler])
        .mount("/registered", routes![read])
        .launch();
}