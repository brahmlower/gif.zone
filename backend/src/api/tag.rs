
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use iron::prelude::*;
// -----------------------------------------------------------------------------
use middleware::PostgresReqExt;
use models::tag::TagId;
use domain::tag as domain;
use api::util;
// -----------------------------------------------------------------------------

pub fn list(req: &mut Request) -> IronResult<Response> {
    let db_conn = req.get_db_conn();
    let result = domain::list(&db_conn);
    util::result_to_ironresult(result)
}

pub fn get(req: &mut Request) -> IronResult<Response> {
    let db_conn = req.get_db_conn();
    let tag_id = util::get_param::<TagId>(req);
    let result = domain::get(&db_conn, &tag_id);
    util::result_to_ironresult(result)
}
