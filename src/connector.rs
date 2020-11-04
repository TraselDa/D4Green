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
use self::cdrs::cluster::NodeTcpConfig;
use self::cdrs::authenticators::Authenticator;
type CurrentSession = Session<RoundRobin<TcpConnectionPool<StaticPasswordAuthenticator>>>;
trait Database_fn{
    fn to_query_values(self)->QueryValues;
}
pub struct Connector{
    address:String,
    port:String,
    user:String,
    password:String,
    no_compression: Option<Session<RoundRobin<TcpConnectionPool<StaticPasswordAuthenticator>>>>,
}
pub struct Region{
    id:i32,
    departement:Departement,
    nom:String

}
pub struct Departement{
    id:i32,
    nom:String,
    code_postal:String
}
pub struct Commune{
    id:i32,
    nom:String,
    code_iris:u32,
    classement:i32,
    nom_iris:String,
    total_population:i32,
    score_global:u32
}
impl Database_fn for Departement{
    fn to_query_values(self) -> QueryValues {
        query_values!("id" => self.id , "nom" => self.nom, "code_postale" => self.code_pastale)
    }
}
impl Database_fn for Region{
    fn to_query_values(self) -> QueryValues {
        query_values!("id" => self.id , "departement" => self.departement.to_query_values(), "nom" => self.nom)
    }
}
pub struct RegionDatabase{
    connector:Connecto,
    _name:String
}
impl RegionDatabase{
    pub fn new(connector:&Connector)->RegionDatabase{
        RegionDatabase{
            connector:connector,
            _name:"Region".to_string()
        }
    }
    pub fn addRegion(self,region:Region){
        self.connector.insert_database(self._name,region.to_query_values());
    }
}

impl Connector{
    pub fn new(username:&str, password:&str) -> Connector {

         let mut connector =Connector{

            address:"146.59.196.39".to_string() ,
            port: "9042".to_string(),
            user: username.to_string(),
            password: password.to_string(),
            no_compression:None
        };
        let auth=StaticPasswordAuthenticator::new(username, password);
        let address=format!("{:?}:{:?}", connector.address, connector.port);
        let node= NodeTcpConfigBuilder::new(&address,auth ).build();
        let cluster_config = ClusterTcpConfig(vec![node]);
        connector.no_compression= Option::from(new_session(&cluster_config, RoundRobin::new()).expect("session should be created"));
        return connector;
    }
    pub fn update_database(self,&name:&String,query:&QueryValues){

    }
    pub fn delete_database(self,name:&String,query:&QueryValues){
    }
    pub fn insert_database(self,name:&String,query:&QueryValues){
        let insert_struct_cql = format!("INSERT INTO eclipsed4g.{:?} \
                     (id,departement,name) VALUES (?)",name);
        self.query_with_values(insert_struct_cql, row.into_query_values())
            .expect("insert");
    }
}