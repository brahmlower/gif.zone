
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
pub struct GifId (pub i32);

impl FromStr for GifId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let int = s.parse::<i32>()?;
        Ok(GifId(int))
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, ToSql, FromSql)]
#[postgres(name = "file_type")]
pub enum FileType {
    Gif,
    Webm
}

impl<'a> From<Row<'a>> for FileType {
    fn from(row: Row) -> Self {
        row.get(0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gif {
    pub id:     GifId,
    pub title:  String,
    pub ftype:  FileType,
    pub fname:  String,
    pub views:  i32
}

impl<'a> From<Row<'a>> for Gif {
    fn from(row: Row) -> Self {
        Gif {
            id:     row.get(0),
            title:  row.get(1),
            ftype:  row.get(2),
            fname:  row.get(3),
            views:  row.get(4)
        }
    }
}
