use cdrs::types::*;
use cdrs::error::Result;
use crate::QueryValues;
use cdrs::types::prelude::*;
use crate::cdrs::frame::*;
use crate::cdrs::types::from_cdrs::FromCDRSByName;
use crate::cdrs::frame::TryFromRow;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow,Serialize, Deserialize)]
pub struct Departement {
    nomdep: String,
    nomr: Option<String>
}
#[derive(Clone, Debug, IntoCDRSValue, TryFromRow,Serialize, Deserialize)]
pub struct DepartementName{
    nomdep: String
}
impl DepartementName{
    pub fn into_query_values(self) -> QueryValues {
            query_values!("nomdep" => self.nomdep)
    }

}
impl Departement {

    pub fn into_query_values(self) -> QueryValues {
        match self.nomr {
            None => return query_values!("nomdep" => self.nomdep),
            Some(p) =>return query_values!("nomdep" => self.nomdep,"nomr" => p),
        }
    }


}



