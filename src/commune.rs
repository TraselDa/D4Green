use crate::QueryValues;
use cdrs::types::prelude::*;
use crate::cdrs::types::from_cdrs::FromCDRSByName;
use crate::cdrs::frame::TryFromRow;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, TryFromRow,Serialize, Deserialize)]
pub struct Commune {
    id:i64,
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
    population:String,
    score_global:String
}

impl Commune {

    pub fn into_query_values(self) -> QueryValues {
        query_values!(
        "nomdep" => self.nomdep,
        "nomr"=>self.nomr,
        "nom_com"=>self.nom_com,
        "acces_a_linformation"=>self.acces_a_linformation,
    "acces_aux_interfaces_numerique"=>self.acces_aux_interfaces_numerique,
    "classement"=>self.classement,
    "competences_administratives"=>self.competences_administratives,
    "competences_numeriques"=>self.competences_numeriques,
    "global_acces"=>self.global_acces,
    "global_competences"=>self.global_competences,
    "nom_iris"=>self.nom_iris,
    "population"=>self.population,
        "code_iris"=>self.code_iris)

    }


}
