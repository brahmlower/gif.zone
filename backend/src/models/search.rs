
// -----------------------------------------------------------------------------
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
// -----------------------------------------------------------------------------
use chrono::prelude::*;
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------

/// TODO: Implement Tag struct
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct SearchQuery {
    pub term: String,
    pub in_caption: Option<Vec<String>>,
    // pub has_tags: Option<Vec<Tag>>
}
