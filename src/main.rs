#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate uuid;
extern crate time;
use rocket_contrib::templates::Template;
use cdrs::query::QueryValues;
use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;
use serde::{Serialize, Deserialize};
use std::vec::*;
use rocket_contrib::{json::{Json}};
use serde_json::Result;
use rocket::data::FromDataSimple;
use rocket::{Request, Data, data, Outcome};
use rocket::http::{ContentType, Status};
use std::io::Read;
use rocket::outcome::Outcome::{Failure, Success};
mod commune;
mod departement;
mod region;
mod db;
#[macro_use]
extern crate rocket;

#[derive(Clone, Debug,Serialize, Deserialize)]
struct DataRS{
    departements_names:Option<std::vec::Vec<departement::DepartementName>>,
    communes_names:Option<std::vec::Vec<commune::CommuneName>>,
    regions_names:Option<std::vec::Vec<region::Region>>,
    total_departements:i32,
    total_communes:i32,
    total_regions:i32,
    bigest_population:Option<commune::Commune>,
    first_classement:Option<commune::Commune>,
    best_indice:Option<commune::Commune>
}


#[get("/")]
fn index() -> Template {
    let mut session = connect_to_db();
    let results_dep=db::select_departements_name(&mut session);
    let results_reg=db::select_regions_name(&mut session);
    let results_com=db::select_communes_name(&mut session);
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
        total_departements: 0,
        total_communes: 0,
        total_regions: 0,
        bigest_population: None,
        first_classement: None,
        communes_names:communes,
        best_indice: None
    };
    context.insert("data",data);
    Template::render("index",&context)
}
#[post("/loadTable",data="<paging>", format="application/x-commune-paging")]
fn loadTable(paging:CommunePaging) -> Json<Vec<commune::Commune>>
{
    let mut session = connect_to_db();
    let result=match paging.region{
        Some(region)=>db::select_commune_by_region(&mut session,region,paging.last_id),
        None=>match paging.departement {
            Some(departement)=>db::select_commune_by_departement(&mut session,departement,paging.last_id),
            None=>match paging.commune{
                Some(commune)=>db::select_commune_by_name(&mut session,commune,paging.last_id),
                None=>db::select_all_communes(&mut session,paging.current_page,paging.last_id)
            }
        }
    };
    let empty:Vec<commune::Commune>=Vec::new();
    if result.is_ok(){
        let communes= result.ok();
        match communes{
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


#[derive(Serialize, Deserialize)]
pub struct CommunePaging{
    current_page:i32,
    last_id:Option<i32>,
    departement:Option<String>,
    region:Option<String>,
    commune:Option<String>,
    search:Option<String>
}
impl CommunePaging{
    pub fn into_query_values(self) -> QueryValues {
        query_values!(
        "current_page" => self.current_page,
        "departement"=>self.departement,
        "commune"=>self.commune,
        "region"=>self.region,
        "last_id"=>self.last_id,
        "search"=>self.search)

    }
}
impl FromDataSimple for CommunePaging {
    type Error = String;
    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        // Ensure the content type is correct before opening the data.
        let person_ct = ContentType::new("application", "x-commune-paging");
        if req.content_type() != Some(&person_ct) {
            return Outcome::Forward(data);
        }

        let mut string = String::new();
        if let Err(e) = data.open().take(256).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));

        }
        let pt:&str= &*string;
         let p= convert_to_paging(pt);
        if p.is_ok(){
            return match p.ok(){
                Some(x)=>Success(x),
                None=>Failure((Status::InternalServerError,"Erreur survenue dans la conversion".to_string()))

            }
        }
        return Failure((Status::InternalServerError, "Erreur survenue dans la conversion".to_string()));
    }
}
pub fn convert_to_paging(data_:&str) -> Result<CommunePaging> {
    let data=data_;
    let v: CommunePaging = serde_json::from_str(data)?;
    return Ok(v);
}


