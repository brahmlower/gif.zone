// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use database::tag as database;
use middleware::PostgresConnection;
use models::error::DomainError;
use models::gif::GifId;
use models::tag::Tag;
use models::tag::TagId;
// -----------------------------------------------------------------------------

pub fn list(conn: &PostgresConnection) -> Result<Vec<Tag>, DomainError> {
    let result = database::fetch_all(&conn)?;
    Ok(result)
}

pub fn get(conn: &PostgresConnection, id: &TagId) -> Result<Tag, DomainError> {
    let result = database::fetch_one(&conn, id)?;
    Ok(result)
}

pub fn get_by_gif(conn: &PostgresConnection, id: &GifId) -> Result<Vec<Tag>, DomainError> {
    let result = database::fetch_by_gif(&conn, id)?;
    Ok(result)
}
