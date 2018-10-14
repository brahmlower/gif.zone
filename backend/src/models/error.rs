// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use postgres::Error as PostgresError;
// -----------------------------------------------------------------------------
// use models::entity::EntityId;
// use models::planet::PlanetId;
// -----------------------------------------------------------------------------

#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum AppError {
    Api(ApiError),
    Database(DatabaseError),
    Domain(DomainError),
}

impl From<PostgresError> for DatabaseError {
    fn from(error: PostgresError) -> Self {
        warn!("QueryFailure: {:?}", error);
        DatabaseError::QueryFailure
    }
}

impl From<DatabaseError> for DomainError {
    fn from(error: DatabaseError) -> Self {
        DomainError::TransactionFailure(error)
    }
}

impl From<ApiError> for AppError {
    fn from(error: ApiError) -> Self {
        AppError::Api(error)
    }
}

impl From<DomainError> for AppError {
    fn from(error: DomainError) -> Self {
        AppError::Domain(error)
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ApiError {
    JsonParseResponseFailed,
}

#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum DatabaseError {
    ConnectionFailure,
    NoItemWithId,
    TooManyItems,
    QueryFailure,
}

#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum DomainError {
    TransactionFailure(DatabaseError),
    BadCredentials,
}
