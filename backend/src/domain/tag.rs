
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use database::tag as database;
use middleware::PostgresConnection;
use models::error::DomainError;
use models::tag::Tag;
use models::tag::TagId;
// use models::search::SearchQuery;
// -----------------------------------------------------------------------------

pub fn list(conn: &PostgresConnection) -> Result<Vec<Tag>, DomainError> {
    let result = database::fetch_all(&conn)?;
    Ok(result)
}

pub fn get(conn: &PostgresConnection, id: TagId) -> Result<Tag, DomainError> {
    let result = database::fetch_one(&conn, &id)?;
    Ok(result)
}
