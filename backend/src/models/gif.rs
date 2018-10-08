
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
pub struct GifId (pub i32);

impl FromStr for GifId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let int = s.parse::<i32>()?;
        Ok(GifId(int))
    }
}

impl UriParam for GifId {
    fn as_uri_param() -> &'static str {
        "gif"
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, ToSql, FromSql)]
#[postgres(name = "file_type")]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Gif,
    Webm
}

impl<'a> From<Row<'a>> for FileType {
    fn from(row: Row) -> Self {
        row.get(0)
    }
}

#[derive(Debug)]
pub struct ParseFileTypeError;

impl Error for ParseFileTypeError {
    fn description(&self) -> &str {
        "Failed to parse string reference to FileType (impossible)"
    }
}

impl Display for ParseFileTypeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "ParseFileTypeError")
    }
}
impl FromStr for FileType {
    type Err = ParseFileTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gif"   => Ok(FileType::Gif),
            "webm"  => Ok(FileType::Webm),
            _ => Err(ParseFileTypeError)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gif {
    pub id:             GifId,
    pub resource_id:    String,
    pub file_type:      FileType,
    pub caption:        String,
    pub views:          i32
}

impl<'a> From<Row<'a>> for Gif {
    fn from(row: Row) -> Self {
        Gif {
            id:             row.get(0),
            resource_id:    row.get(1),
            file_type:      row.get(2),
            caption:        row.get(3),
            views:          row.get(4)
        }
    }
}
