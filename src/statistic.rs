use serde::{Serialize, Deserialize};
use crate::commune::Commune;
use crate::region::Region;
use crate::cdrs::frame::TryFromRow;
use cdrs::types::prelude::Row;
use cdrs::types::prelude::UDT;
use crate::cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::query::QueryValues;


