// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use postgres::Connection;
// -----------------------------------------------------------------------------
use database::util::producing_list;
use database::util::producing_one;
use models::error::DatabaseError;
use models::gif::Gif;
use models::gif::GifId;
use models::search::SearchQuery;
// -----------------------------------------------------------------------------

const SQL_SELECT_ALL: &str = "SELECT id, \
                              resource_id, \
                              file_type, \
                              caption, \
                              views \
                              FROM gif";

const SQL_SELECT_ONE: &str = "SELECT id, \
                              resource_id, \
                              file_type, \
                              caption, \
                              views \
                              FROM gif \
                              WHERE id = $1";

const SQL_SELECT_FILTER: &str = "SELECT id, \
                                 resource_id, \
                                 file_type, \
                                 caption, \
                                 views \
                                 FROM gif \
                                 WHERE caption ILIKE '%' || $1 || '%'";

pub fn fetch_all(conn: &Connection) -> Result<Vec<Gif>, DatabaseError> {
    producing_list(conn, SQL_SELECT_ALL, Box::new([]))
}

pub fn fetch_one(conn: &Connection, id: &GifId) -> Result<Gif, DatabaseError> {
    let &GifId(id) = id;
    producing_one(conn, SQL_SELECT_ONE, Box::new([&id]))
}

pub fn fetch_filter(conn: &Connection, query: &SearchQuery) -> Result<Vec<Gif>, DatabaseError> {
    let cap_value = &query.cap_value.clone().unwrap_or_else(|| "".to_owned());
    producing_list(conn, SQL_SELECT_FILTER, Box::new([&cap_value]))
}
