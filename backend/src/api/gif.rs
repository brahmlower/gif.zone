
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use iron::prelude::*;
use iron::status;
// -----------------------------------------------------------------------------
use middleware::PostgresReqExt;
use models::error::AppError;
use domain::gif as domain;
use super::util;
// -----------------------------------------------------------------------------

pub fn list(req: &mut Request) -> IronResult<Response> {
    // Pull a database connection from the db pool
    let db_conn = req.get_db_conn();
    // Call the domain function
    let result = domain::list(&db_conn);

    // Build iron response from domain result
    match result {
        Ok(content) => util::to_json_response(content),
        Err(error)  => util::to_json_error(AppError::from(error))
    }
}

pub fn get(req: &mut Request) -> IronResult<Response> {
    // Pull a database connection from the db pool
    let db_conn = req.get_db_conn();
    // Call the domain function
    let result = domain::list(&db_conn);

    // Build iron response from domain result
    match result {
        Ok(content) => util::to_json_response(content),
        Err(error)  => util::to_json_error(AppError::from(error))
    }
}