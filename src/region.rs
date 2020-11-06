use crate::QueryValues;
use cdrs::types::prelude::*;
use crate::cdrs::frame::*;
use crate::cdrs::types::from_cdrs::FromCDRSByName;
use crate::cdrs::frame::TryFromRow;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq,Serialize, Deserialize)]
pub struct Region {
    nomr: String
}
impl Region{
    pub fn into_query_values(self) -> QueryValues {
        query_values!(
                "nomr" => self.nomr
          )
    }
}