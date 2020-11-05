use cdrs::types::*;
use cdrs::error::Result;
use crate::QueryValues;
use cdrs::types::prelude::*;
use crate::cdrs::frame::*;
use crate::cdrs::types::from_cdrs::FromCDRSByName;
use crate::cdrs::frame::TryFromRow;

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
pub struct Departement {
    nom: String,
    code_postal: String
}

impl Departement {

    pub fn into_query_values(self) -> QueryValues {
        query_values!(
      "nom" => self.nom,
      "code_postal" => self.code_postal
          )
    }

}



