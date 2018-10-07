
// -----------------------------------------------------------------------------
use std::num::ParseIntError;
use std::str::FromStr;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
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

#[derive(Debug)]
pub struct ParseTagNameError;

impl Error for ParseTagNameError {
    fn description(&self) -> &str {
        "Failed to parse string reference to TagName (impossible)"
    }
}

impl Display for ParseTagNameError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "ParseTagNameError")
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, ToSql, FromSql)]
pub struct TagName (pub String); // TODO: It'd be cool if we could just use a `str`

impl FromStr for TagName {
    type Err = ParseTagNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Err(ParseTagNameError),
            _ => Ok(TagName(s.to_owned()))
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, ToSql, FromSql)]
pub struct Tag {
    pub id:     TagId,
    pub name:   TagName
}

impl<'a> From<Row<'a>> for Tag {
    fn from(row: Row) -> Self {
        Tag {
            id:     row.get(0),
            name:   row.get(1)
        }
    }
}
