
// -----------------------------------------------------------------------------
use std::str::FromStr;
// -----------------------------------------------------------------------------
use iron::prelude::*;
use params::Map;
use params::Params;
use params::Value;
// -----------------------------------------------------------------------------
use middleware::PostgresReqExt;
use models::gif::GifId;
use models::gif::FileType;
use models::tag::TagName;
use models::search::SearchQuery;
use domain::gif as domain;
use domain::tag as tag_domain;
use api::util;
// -----------------------------------------------------------------------------
use iron::status;

/// Gets all gifs
pub fn list(req: &mut Request) -> IronResult<Response> {
    let db_conn = req.get_db_conn();
    let result = domain::list(&db_conn);
    util::result_to_ironresult(result)
}

/// Gets a particular gif
pub fn get(req: &mut Request) -> IronResult<Response> {
    let db_conn = req.get_db_conn();
    let gif_id = util::get_param::<GifId>(req);
    let result = domain::get(&db_conn, &gif_id);
    util::result_to_ironresult(result)
}

/// Gets tags associated with a particular gif
pub fn get_tags(req: &mut Request) -> IronResult<Response> {
    let db_conn = req.get_db_conn();
    let gif_id = util::get_param::<GifId>(req);
    let result = tag_domain::get_by_gif(&db_conn, &gif_id);
    util::result_to_ironresult(result)
}

/// Search gifs
pub fn search(req: &mut Request) -> IronResult<Response> {
    let db_conn = req.get_db_conn();
    let map = req.get_ref::<Params>().unwrap();
    let query = map_to_search(map);
    debug!("{:?}", query);
    let result = domain::search(&db_conn, &query);
    util::result_to_ironresult(result)
}

fn map_to_search(query: &Map) -> SearchQuery {
    let cap_only: Option<bool> = match query.find(&["cap_only"]) {
        Some(&Value::Boolean(ref b)) => Some(*b),
        Some(_) => None,
        None => None,
    };
    let cap_value: Option<String> = match query.find(&["cap_value"]) {
        Some(&Value::String(ref s)) => Some(s.to_string()),
        Some(_) => None,
        None => None,
    };
    let file_types: Option<Vec<FileType>> = match query.find(&["file_types"]) {
        Some(&Value::String(ref s)) => {
            let values = s.split(",");
            let ftype_iter = values.filter_map(|i| FileType::from_str(i).ok());
            Some(ftype_iter.collect())
        },
        Some(_) => None,
        None => None,
    };
    let tags: Option<Vec<TagName>>= match query.find(&["tags"]) {
        Some(&Value::String(ref s)) => {
            let values = s.split(",");
            let tag_iter = values.filter_map(|i| TagName::from_str(i).ok());
            Some(tag_iter.collect())
        },
        Some(_) => None,
        None => None,
    };

    SearchQuery {
        cap_only:   cap_only,
        cap_value:  cap_value,
        file_types: file_types,
        tags:       tags
    }
}
