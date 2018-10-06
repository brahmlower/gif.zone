
// -----------------------------------------------------------------------------
use std::num::ParseIntError;
use std::str::FromStr;
// -----------------------------------------------------------------------------
use postgres::rows::Row;
// -----------------------------------------------------------------------------
use models::UriParam;
// -----------------------------------------------------------------------------

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, ToSql, FromSql)]
pub struct TagId (pub i32);

impl FromStr for TagId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let int = s.parse::<i32>()?;
        Ok(TagId(int))
    }
}

impl UriParam for TagId {
    fn as_uri_param() -> &'static str {
        "tag"
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, ToSql, FromSql)]
pub struct Tag {
    pub id:     TagId,
    pub name:   String
}

impl<'a> From<Row<'a>> for Tag {
    fn from(row: Row) -> Self {
        Tag {
            id:     row.get(0),
            name:   row.get(1)
        }
    }
}
