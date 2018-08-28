
// -----------------------------------------------------------------------------
// use std::collections::HashMap;
// use std::num::ParseIntError;
// use std::str::FromStr;
// -----------------------------------------------------------------------------
// use chrono::prelude::*;
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq, Deserialize, Debug)]
pub enum FileType {
    Any,
    Gif,
    Webm
}

#[derive(Clone, PartialEq, Eq, Deserialize, Debug)]
pub enum Captions {
    Any,
    Yes,
    No
}

#[derive(Clone, PartialEq, Eq, Deserialize, Debug)]
pub struct SearchQuery {
    pub captions: Captions, // swap out for Option<Bool> at some point
    pub ftype: FileType,    // swap out for Option<Bool> at some point (maybe. depends if we have mroe than just gif/webm)
    pub labels: Vec<String>,
    pub value: String
}
