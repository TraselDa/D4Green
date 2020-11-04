#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
mod connector;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
   let context:HashMap<u32,u32> = HashMap::new();
    Template::render("index",&context)

}

fn main() {


    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index]).launch();
}
