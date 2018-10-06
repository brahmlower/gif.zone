
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use iron::prelude::*;
// -----------------------------------------------------------------------------
use middleware::PostgresReqExt;
use models::gif::GifId;
use models::search::SearchQuery;
use domain::gif as domain;
use super::util;
// -----------------------------------------------------------------------------

/// Gets all gifs. This should rarely be used I think
pub fn list(req: &mut Request) -> IronResult<Response> {
    let db_conn = req.get_db_conn();
    let result = domain::list(&db_conn);
    util::result_to_ironresult(result)
}

/// Gets a single gif
pub fn get(req: &mut Request) -> IronResult<Response> {
    let db_conn = req.get_db_conn();
    let gif_id = util::get_param::<GifId>(req);
    let result = domain::get(&db_conn, gif_id);
    util::result_to_ironresult(result)
}

/// Search gifs
pub fn search(req: &mut Request) -> IronResult<Response> {
    let db_conn = req.get_db_conn();
    let body = match util::parse_body::<SearchQuery>(req) {
        Ok(b) => b,
        Err(e) => return e
    };
    let result = domain::search(&db_conn, &body);
    util::result_to_ironresult(result)
}
