// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use database::gif as database;
use middleware::PostgresConnection;
use models::error::DomainError;
use models::gif::Gif;
use models::gif::GifId;
use models::search::SearchQuery;
// -----------------------------------------------------------------------------

pub fn list(conn: &PostgresConnection) -> Result<Vec<Gif>, DomainError> {
    info!("Getting all gifs");
    let result = database::fetch_all(&conn)?;
    Ok(result)
}

pub fn get(conn: &PostgresConnection, id: &GifId) -> Result<Gif, DomainError> {
    info!("Getting gif with id {:?}", id);
    let result = database::fetch_one(&conn, id)?;
    Ok(result)
}

pub fn search(conn: &PostgresConnection, query: &SearchQuery) -> Result<Vec<Gif>, DomainError> {
    info!("Searching gifs using query: {:?}", query);
    let result = database::fetch_filter(&conn, query)?;
    Ok(result)
}
