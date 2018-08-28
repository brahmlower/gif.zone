
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use bodyparser;
use iron::prelude::*;
// use iron::status;
// -----------------------------------------------------------------------------
use middleware::PostgresReqExt;
use models::error::AppError;
// use models::error::DomainError;
use models::search::SearchQuery;
use domain::gif as domain;
use super::util;
// -----------------------------------------------------------------------------

/// Gets all gifs. This should rarely be used I think
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

/// Gets a single gif
pub fn get(req: &mut Request) -> IronResult<Response> {
    // Pull a database connection from the db pool
    let db_conn = req.get_db_conn();
    let gif_id = util::get_param_gif(req);
    // Call the domain function
    let result = domain::get(&db_conn, gif_id);

    // Build iron response from domain result
    match result {
        Ok(content) => util::to_json_response(content),
        Err(error)  => util::to_json_error(AppError::from(error))
    }
}

/// Search gifs
pub fn search(req: &mut Request) -> IronResult<Response> {
    // Pull a database connection from the db pool
    let db_conn = req.get_db_conn();
    // Parse SearchQuery from request body
    let query = req.get::<bodyparser::Struct<SearchQuery>>().unwrap().unwrap();
    // Call the domain function
    let result = domain::search(&db_conn, &query);

    // Build iron response from domain result
    util::result_to_ironresult(result)
}
