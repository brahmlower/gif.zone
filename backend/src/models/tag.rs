
// -----------------------------------------------------------------------------
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
// -----------------------------------------------------------------------------
use postgres::rows::Row;
use chrono::prelude::*;
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, ToSql, FromSql)]
pub struct TagId (pub i32);

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id:     TagId,
    pub name:   String
}
