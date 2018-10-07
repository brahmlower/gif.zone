
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use models::gif::FileType;
use models::tag::TagName;
// -----------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq, Deserialize, Debug)]
pub struct SearchQuery {
    pub cap_only:   Option<bool>,
    pub cap_value:  Option<String>,
    pub file_types: Option<Vec<FileType>>,
    pub tags:       Option<Vec<TagName>>
}
