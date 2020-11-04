#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::templates::Template;
use std::collections::HashMap;
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
   let mut context:HashMap<u32,u32> = HashMap::new();
    Template::render("index",&context)    

}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index]).launch();
}
