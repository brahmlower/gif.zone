
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use postgres::Connection;
// -----------------------------------------------------------------------------
use database::util::producing_list;
use database::util::producing_one;
use models::error::DatabaseError;
use models::gif::GifId;
use models::gif::Gif;
// -----------------------------------------------------------------------------

const SQL_SELECT_ALL: &str =
    "SELECT id, \
            title, \
            ftype, \
            views
    FROM    gif";

const SQL_SELECT_ONE: &str =
    "SELECT id, \
            title, \
            ftype, \
            views \
    FROM    gif \
    WHERE   id = $1";

pub fn fetch_all(conn: &Connection) -> Result<Vec<Gif>, DatabaseError> {
    producing_list(conn, SQL_SELECT_ALL, Box::new([]))
}

pub fn fetch_one(conn: &Connection, id: &GifId) -> Result<Gif, DatabaseError> {
    let &GifId(id) = id;
    producing_one(conn, SQL_SELECT_ONE, Box::new([ &id ]))
}
