#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::templates::Template;
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
   let context = "hello world";
    Template::render("index", &context)    

}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index]).launch();
}
