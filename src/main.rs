#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate uuid;
extern crate time;
use rocket_contrib::templates::Template;
use cdrs::frame::traits::*;
use cdrs::query::QueryValues;
use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;
mod departement;
mod db;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
   let mut context:HashMap<u32,u32> = HashMap::new();
    Template::render("index",&context)

}

fn main() {
    //let mut session = connect_to_db();
    rocket::ignite()
        .mount("/static",StaticFiles::from("./static"))
        .attach(Template::fairing())
        .mount("/", routes![index]).launch();

}

fn connect_to_db() -> db::CurrentSession {
    let mut session = db::create_db_session().expect("create db session error");
    db::create_keyspace(&mut session).expect("create keyspace error");
    session
}