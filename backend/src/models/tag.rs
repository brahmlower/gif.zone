// -----------------------------------------------------------------------------
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::num::ParseIntError;
use std::str::FromStr;
// -----------------------------------------------------------------------------
use postgres::rows::Row;
// -----------------------------------------------------------------------------
use models::UriParam;
// -----------------------------------------------------------------------------

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, ToSql, FromSql)]
pub struct TagId(pub i32);

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
pub struct ParseTagLabelError;

impl Error for ParseTagLabelError {
    fn description(&self) -> &str {
        "Failed to parse string reference to TagLabel (impossible)"
    }
}

impl Display for ParseTagLabelError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "ParseTagLabelError")
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, ToSql, FromSql)]
pub struct TagLabel(pub String); // TODO: It'd be cool if we could just use a `str`

impl FromStr for TagLabel {
    type Err = ParseTagLabelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Err(ParseTagLabelError),
            _ => Ok(TagLabel(s.to_owned())),
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, ToSql, FromSql)]
pub struct Tag {
    pub id: TagId,
    pub label: TagLabel,
}

impl<'a> From<Row<'a>> for Tag {
    fn from(row: Row) -> Self {
        Tag {
            id: row.get(0),
            label: row.get(1),
        }
    }
}
