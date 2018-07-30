
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use database::gif as database;
use middleware::PostgresConnection;
use models::error::DomainError;
use models::gif::Gif;
use models::gif::GifId;
// -----------------------------------------------------------------------------

pub fn list(conn: &PostgresConnection) -> Result<Vec<Gif>, DomainError> {
    let result = database::fetch_all(&conn)?;
    Ok(result)
}

pub fn get(conn: &PostgresConnection, id: GifId) -> Result<Gif, DomainError> {
    let result = database::fetch_one(&conn, &id)?;
    Ok(result)
} 
