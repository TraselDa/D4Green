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
use rocket::http::{ContentType, Status, RawStr};
use std::io::Read;
use rocket::outcome::Outcome::{Failure, Success};
mod commune;
mod departement;
mod region;
mod db;
use rocket::response::NamedFile;
use std::path::Path;

#[macro_use]
extern crate rocket;

#[derive(Clone, Debug,Serialize, Deserialize)]
struct DataRS{
    regions_names:Option<std::vec::Vec<region::Region>>,
}


#[get("/")]
fn index() -> Template {
    let mut session = connect_to_db();
    let results_reg=db::select_regions_name(&mut session);
    let mut regions=None;
   
    if results_reg.is_ok(){
        regions=results_reg.ok();
    }
    let mut context:HashMap<&str, _ > = HashMap::new();
    let data=DataRS{
        regions_names:regions,
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

#[get("/download/<id>")]
fn download(id:&RawStr) -> std::io::Result<NamedFile> {
    let mut session = connect_to_db();
    let path = Path::new("static/").join(id.as_str().to_string()).join(".pdf");
/*
    let result_commune=db::get_commune(&mut session,id.as_str().to_string());
    if result_commune.is_ok(){
        let commune=result_commune.ok();
        match commune{
            Some(com)=>{

                let result_departement=db::get_departement(&mut session,com.get_departement());
                if result_departement.is_ok(){
                    let (doc, page1, layer1) = PdfDocument::new(format!("Document commune {}",com.get_nom()), Mm(247.0), Mm(210.0), "Layer 1");
                    let current_layer = doc.get_page(page1).get_layer(layer1);
                    current_layer.begin_text_section();
                    let font = doc.add_builtin_font(BuiltinFont::TimesBold).unwrap();
                    current_layer.write_text("",&font);
                    doc.save(&mut BufWriter::new(File::create(path).unwrap())).unwrap();
                };
            }
            ,
            None=>{}
        }


    };*/
    NamedFile::open(&path)
}

fn main() {
    rocket::ignite()
        .mount("/static",StaticFiles::from("./static"))
        .attach(Template::fairing())
        .mount("/", routes![index,loadTable,download]).launch();

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


