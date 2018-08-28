
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
// use bodyparser;
use iron::prelude::*;
// use iron::status;
// -----------------------------------------------------------------------------
use middleware::PostgresReqExt;
// use models::error::AppError;
// use models::error::DomainError;
use domain::tag as domain;
use super::util;
// -----------------------------------------------------------------------------

pub fn list(req: &mut Request) -> IronResult<Response> {
    let db_conn = req.get_db_conn();
    let result = domain::list(&db_conn);
    // Build iron response from domain result
    util::result_to_ironresult(result)
}

pub fn get(req: &mut Request) -> IronResult<Response> {
    let db_conn = req.get_db_conn();
    let tag_id = util::get_param_tag(req);
    let result = domain::get(&db_conn, tag_id);
    util::result_to_ironresult(result)
}
