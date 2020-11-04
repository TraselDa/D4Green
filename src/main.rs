#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::templates::Template;
use std::collections::HashMap;
extern crate cdrs;
use cdrs::authenticators::StaticPasswordAuthenticator;
use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::{new as new_session,Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder,TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;

use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;

type CurrentSession = Session<RoundRobin<TcpConnectionPool<StaticPasswordAuthenticator>>>;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
   let mut context:HashMap<u32,u32> = HashMap::new();
    Template::render("index",&context)    

}

fn main() {
    let user = "user";
    let password = "password";
    let auth = StaticPasswordAuthenticator::new(&user, &password);
    let node = NodeTcpConfigBuilder::new("localhost:9042", auth).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let no_compression: CurrentSession =
        new_session(&cluster_config, RoundRobin::new()).expect("session should be created");


    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index]).launch();
}
