// -----------------------------------------------------------------------------
use std::fmt::Debug;
use std::str::FromStr;
// use std::fmt::Display;
// -----------------------------------------------------------------------------
use bodyparser;
use iron::prelude::*;
// use iron::headers::Authorization;
// use iron::headers::Bearer;
use iron::status;
use iron::status::Status;
use router::Router;
// use jwt::decode;
// use jwt::Validation;
// use serde;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
// -----------------------------------------------------------------------------
use models::error::ApiError;
use models::error::AppError;
use models::error::DatabaseError;
use models::error::DomainError;
use models::UriParam;
// -----------------------------------------------------------------------------

/// TODO: There must be a better way of handling this. Is there no way to
/// implement `To` or `From` which will take `Result<T: Serialize, DomainError>`
/// and return a `IronResult<Response>`?
pub fn result_to_ironresult<T: Serialize>(result: Result<T, DomainError>) -> IronResult<Response> {
    match result {
        Ok(content) => to_json_response(content),
        Err(error) => to_json_error(AppError::from(error)),
    }
}

pub fn to_json_response<T: Serialize>(content: T) -> IronResult<Response> {
    let s = serde_json::to_string(&content);
    match s {
        Ok(t) => Ok(Response::with((status::Ok, t))),
        Err(_) => Ok(Response::with((status::InternalServerError, ""))),
    }
}

// Error handling functions ----------------------------------------------------

/// Converts AppError to a JSON IronResult
pub fn to_json_error(error: AppError) -> IronResult<Response> {
    let response_tup = match error {
        AppError::Api(ref app_e) => to_json_error_api(&error, app_e),
        AppError::Database(ref db_e) => to_json_error_database(&error, db_e),
        AppError::Domain(ref dom_e) => to_json_error_domain(&error, dom_e),
    };
    Ok(Response::with(response_tup))
}

fn to_json_error_api(app_e: &AppError, api_e: &ApiError) -> (Status, String) {
    let s = serde_json::to_string(app_e).unwrap();
    match api_e {
        &ApiError::JsonParseResponseFailed => (status::InternalServerError, s),
    }
}

fn to_json_error_database(app_e: &AppError, db_e: &DatabaseError) -> (Status, String) {
    let s = serde_json::to_string(app_e).unwrap();
    match db_e {
        &DatabaseError::ConnectionFailure => (status::InternalServerError, s),
        &DatabaseError::NoItemWithId => (status::BadRequest, s),
        &DatabaseError::TooManyItems => (status::InternalServerError, s),
        &DatabaseError::QueryFailure => (status::InternalServerError, s),
    }
}

fn to_json_error_domain(app_e: &AppError, dom_e: &DomainError) -> (Status, String) {
    let s = serde_json::to_string(app_e).unwrap();
    match dom_e {
        // &DomainError::EntityIsNotPlanetOwner(_entity, _planet) => (status::BadRequest, s),
        &DomainError::TransactionFailure(_) => (status::InternalServerError, s),
        &DomainError::BadCredentials => (status::Unauthorized, "".to_owned()),
    }
}

// Auth helpers ----------------------------------------------------------------

// pub fn check_auth(req: &mut Request) -> Option<AccountJwt> {
//     let auth_header = req.headers.get::<Authorization<Bearer>>();
//     match auth_header {
//         Some(header) => {
//             let result = decode::<AccountJwt>(&header.token, "secret".as_ref(), &Validation::default());
//             match result {
//                 Ok(token)  => Some(token.claims),
//                 Err(error) => None
//             }
//         },
//         None => None
//     }
// }

// pub fn check_auth_(req: &mut Request) -> Result<AccountJwt, Response> {
//     const UNAUTHORIZED: Response = Response::with((status::Unauthorized, ""));

//     let auth_header = req.headers.get::<Authorization<Bearer>>();
//     match auth_header {
//         Some(header) => {
//             let result = decode::<AccountJwt>(&header.token, "secret".as_ref(), &Validation::default());
//             match result {
//                 Ok(token) => Ok(token.claims),
//                 Err(_)    => Err(UNAUTHORIZED)
//             }
//         },
//         None => Err(UNAUTHORIZED)
//     }
// }

// URI/body getter functions --------------------------------------------------------

pub fn parse_body<T: 'static + for<'a> Deserialize<'a> + Clone>(
    req: &mut Request,
) -> Result<T, IronResult<Response>> {
    let body_result = req.get::<bodyparser::Struct<T>>();
    match body_result {
        Ok(Some(t)) => Ok(t),
        Ok(None) => Err(Ok(Response::with((status::InternalServerError, "No body")))),
        Err(_) => Err(Ok(Response::with((
            status::InternalServerError,
            "Error while getting request body",
        )))),
    }
}

/// Get url prarmeter using the types implementation of UriParam
pub fn get_param<T: FromStr + Debug + UriParam>(req: &mut Request) -> T
where
    <T as FromStr>::Err: Debug,
{
    let uri_id = T::as_uri_param();
    let id_str = req
        .extensions
        .get::<Router>()
        .unwrap()
        .find(uri_id)
        .unwrap();
    id_str.parse().unwrap()
}

/// Get url parameter using a provided parameter string
pub fn get_param_as<T: FromStr + Debug>(req: &mut Request, param: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    let id_str = req.extensions.get::<Router>().unwrap().find(param).unwrap();
    id_str.parse().unwrap()
}
