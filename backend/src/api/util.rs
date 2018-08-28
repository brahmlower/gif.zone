
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use iron::prelude::*;
// use iron::headers::Authorization;
// use iron::headers::Bearer;
use iron::status;
use iron::status::Status;
use router::Router;
// use jwt::decode;
// use jwt::Validation;
use serde;
use serde_json;
// -----------------------------------------------------------------------------
use models::gif::GifId;
use models::tag::TagId;
use models::error::AppError;
use models::error::ApiError;
use models::error::DatabaseError;
use models::error::DomainError;
// -----------------------------------------------------------------------------

/// TODO: There must be a better way of handling this. Is there no way to
/// implement `To` or `From` which will take `Result<T: Serialize, DomainError>`
/// and return a `IronResult<Response>`?
pub fn result_to_ironresult<T: serde::Serialize>(result: Result<T, DomainError>) -> IronResult<Response> {
    match result {
        Ok(content) => to_json_response(content),
        Err(error)  => to_json_error(AppError::from(error))
    }
}

pub fn to_json_response<T: serde::Serialize>(content: T) -> IronResult<Response> {
    let s = serde_json::to_string(&content);
    match s {
        Ok(t)  => Ok(Response::with( (status::Ok, t) )),
        Err(_) => Ok(Response::with( (status::InternalServerError, "") ))
    }
}

// Error handling functions ----------------------------------------------------

/// Converts AppError to a JSON IronResponse
pub fn to_json_error(error: AppError) -> IronResult<Response> {
    let response_tup = match error {
        AppError::Api(ref app_e)     => to_json_error_api(&error, app_e),
        AppError::Database(ref db_e) => to_json_error_database(&error, db_e),
        AppError::Domain(ref dom_e)  => to_json_error_domain(&error, dom_e)
    };
    Ok(Response::with(response_tup))
}

fn to_json_error_api(app_e: &AppError, api_e: &ApiError) -> (Status, String) {
    let s = serde_json::to_string(app_e).unwrap();
    match api_e {
        &ApiError::JsonParseResponseFailed => (status::InternalServerError, s)
    }
}

fn to_json_error_database(app_e: &AppError, db_e: &DatabaseError) -> (Status, String) {
    let s = serde_json::to_string(app_e).unwrap();
    match db_e {
        &DatabaseError::ConnectionFailure => (status::InternalServerError, s),
        &DatabaseError::NoItemWithId => (status::BadRequest, s),
        &DatabaseError::TooManyItems => (status::InternalServerError, s),
        &DatabaseError::QueryFailure => (status::InternalServerError, s)
    }
}

fn to_json_error_domain(app_e: &AppError, dom_e: &DomainError) -> (Status, String) {
    let s = serde_json::to_string(app_e).unwrap();
    match dom_e {
        // &DomainError::EntityIsNotPlanetOwner(_entity, _planet) => (status::BadRequest, s),
        &DomainError::TransactionFailure(_) => (status::InternalServerError, s),
        &DomainError::BadCredentials => (status::Unauthorized, "".to_owned())
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

// URI getter functions --------------------------------------------------------

// pub fn get_param_account(req: &mut Request) -> AccountId {
//     let id_str = req.extensions.get::<Router>().unwrap().find("account").unwrap();
//     id_str.parse().unwrap()
// }

pub fn get_param_gif(req: &mut Request) -> GifId {
    let id_str = req.extensions.get::<Router>().unwrap().find("gif").unwrap();
    id_str.parse().unwrap()
}

pub fn get_param_tag(req: &mut Request) -> TagId {
    let id_str = req.extensions.get::<Router>().unwrap().find("tag").unwrap();
    id_str.parse().unwrap()
}
