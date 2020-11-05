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
use std::result::Result;
use rocket_contrib::serve::StaticFiles;
use serde::{Serialize, Deserialize};
use rocket::Data;
use std::vec::*;
use rocket::response::content;
use std::ptr::null;
use rocket_contrib::{json::{Json}};
use rocket_contrib::json::JsonError;
use rocket::http::RawStr;

mod departement;
mod region;
mod commune;
mod db;
#[macro_use] extern crate rocket;

#[derive(Clone, Debug,Serialize, Deserialize)]
struct DataRS{
    departements_names:Option<std::vec::Vec<departement::DepartementName>>,
    communes:Option<std::vec::Vec<commune::Commune>>,
    regions_names:Option<std::vec::Vec<region::Region>>
}
#[get("/")]
fn index() -> Template {
    let mut session = connect_to_db();
    let results_dep=db::select_departements_name(&mut session);
    let results_reg=db::select_regions_name(&mut session);
    let results_com=db::select_all_communes(&mut session);
    let mut departements=None;
    let mut regions=None;
    let mut communes=None;
    if results_dep.is_ok(){
        departements= results_dep.ok();
    }
    if results_reg.is_ok(){
        regions=results_reg.ok();
    }
    if results_com.is_ok(){
        communes=results_com.ok();
    }
    let mut context:HashMap<&str, _ > = HashMap::new();
    let data=DataRS{
        departements_names:departements,
        regions_names:regions,
        communes:communes
    };
    context.insert("data",data);
    Template::render("index",&context)
}
#[post("/loadTable")]
fn loadTable()-> Json<Vec<departement::Departement>>
{
    let mut session = connect_to_db();
    let result=db::select_departements(&mut session,20);
    let mut empty:Vec<departement::Departement>=Vec::new();

    if result.is_ok(){
        let departements= result.ok();

        match departements{
            Some(x)=>return Json(x),
            None=>{}
        }

    }
    return Json(empty)
}
fn main() {
    rocket::ignite()
        .mount("/static",StaticFiles::from("./static"))
        .attach(Template::fairing())
        .mount("/", routes![index,loadTable]).launch();

}

fn connect_to_db() -> db::CurrentSession {
    let mut session = db::create_db_session().expect("create db session error");
    db::create_keyspace(&mut session).expect("create keyspace error");
    session
}