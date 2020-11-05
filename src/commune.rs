use cdrs::types::*;
use cdrs::error::Result;
use crate::QueryValues;
use cdrs::types::prelude::*;
use crate::cdrs::frame::*;
use crate::cdrs::types::from_cdrs::FromCDRSByName;
use crate::cdrs::frame::TryFromRow;
use serde::{Serialize, Deserialize};
use rocket_contrib::{json::{Json}};

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow,Serialize, Deserialize)]
pub struct Commune {
    nom_com:String,
    code_iris:String,
    acces_a_linformation:String,
    acces_aux_interfaces_numerique:String,
    classement:String,
    competences_administratives:String,
    competences_numeriques:String,
    global_acces:String,
    global_competences:String,
    nom_iris:String,
    nomdep:String,
    nomr:String,
    population:i32,
    score_global:i32
}
#[derive(Clone, Debug, IntoCDRSValue, TryFromRow,Serialize, Deserialize)]
pub struct CommunePaging{
    current_page:i32,
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
        "search"=>self.search)

    }
}
impl Commune {

    pub fn into_query_values(self) -> QueryValues {
        query_values!("nomdep" => self.nomdep,
        "nomr"=>self.nomr,
        "nom_com"=>self.nom_com,
        "code_iris"=>self.code_iris)

    }


}
