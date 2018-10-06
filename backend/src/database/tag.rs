
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use postgres::Connection;
// -----------------------------------------------------------------------------
use database::util::producing_list;
use database::util::producing_one;
use models::error::DatabaseError;
use models::gif::GifId;
use models::tag::TagId;
use models::tag::Tag;
// -----------------------------------------------------------------------------

const SQL_SELECT_ALL: &str =
    "SELECT id, \
            label \
    FROM    tag";

const SQL_SELECT_ONE: &str =
    "SELECT id, \
            label \
    FROM    tag \
    WHERE   id = $1";

const SQL_SELECT_BY_GIF: &str =
    "SELECT tag.id, \
            tag.label \
    FROM    tag, \
            gif_tags \
    WHERE   gif_tags.gif = $1 \
        AND gif_tags.tag = tag.id";

pub fn fetch_all(conn: &Connection) -> Result<Vec<Tag>, DatabaseError> {
    producing_list(conn, SQL_SELECT_ALL, Box::new([]))
}

pub fn fetch_one(conn: &Connection, id: &TagId) -> Result<Tag, DatabaseError> {
    let &TagId(id) = id;
    producing_one(conn, SQL_SELECT_ONE, Box::new([ &id ]))
}

pub fn fetch_by_gif(conn: &Connection, id: &GifId) -> Result<Vec<Tag>, DatabaseError> {
    let &GifId(id) = id;
    producing_list(conn, SQL_SELECT_BY_GIF, Box::new([ &id ]))
}
