
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use postgres::Connection;
// -----------------------------------------------------------------------------
use database::util::producing_list;
use database::util::producing_one;
use models::error::DatabaseError;
use models::tag::TagId;
use models::tag::Tag;
// -----------------------------------------------------------------------------

const SQL_SELECT_ALL: &str =
    "SELECT id, \
            tag \
    FROM    tag";

const SQL_SELECT_ONE: &str =
    "SELECT id, \
            tag \
    FROM    tag
    WHERE   id = $1";

pub fn fetch_all(conn: &Connection) -> Result<Vec<Tag>, DatabaseError> {
    producing_list(conn, SQL_SELECT_ALL, Box::new([]))
}

pub fn fetch_one(conn: &Connection, id: &TagId) -> Result<Tag, DatabaseError> {
    let &TagId(id) = id;
    producing_one(conn, SQL_SELECT_ONE, Box::new([ &id ]))
}
