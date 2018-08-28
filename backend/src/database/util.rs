
// -----------------------------------------------------------------------------
use std::cmp::Eq;
use std::hash::Hash;
use std::collections::HashMap;
// -----------------------------------------------------------------------------
use postgres::Connection;
use postgres::types::ToSql;
use postgres::types::FromSql;
use postgres::rows::Row;
use postgres::stmt::Statement;
// -----------------------------------------------------------------------------
use models::error::DatabaseError;
// -----------------------------------------------------------------------------

pub fn producing_list<T: for<'a> From<Row<'a>>>(conn: &Connection, sql: &str, params: Box<[&ToSql]>) -> Result<Vec<T>, DatabaseError> {
    let rows = conn.query(sql, &params)?;
    let vec = rows.iter().map(T::from).collect();
    Ok(vec)
}

// /// Varient of `producting_list` but takes a Statement rather than &str
// pub fn producing_list_<T: for<'a> From<Row<'a>>>(statement: Statement, params: Box<[&ToSql]>) -> Result<Vec<T>, DatabaseError> {
//     let rows = statement.query(&params)?;
//     let vec = rows.iter().map(T::from).collect();
//     Ok(vec)
// }

pub fn producing_one<T: for<'a> From<Row<'a>>>(conn: &Connection, sql: &str, params: Box<[&ToSql]>) -> Result<T, DatabaseError> {
    let rows = conn.query(sql, &params)?;
    match rows.len() {
        0 => Err(DatabaseError::NoItemWithId),
        1 => Ok(T::from(rows.get(0))),
        _ => Err(DatabaseError::TooManyItems)
    }
}

// pub fn producing_unit(conn: &Connection, sql: &str, params: Box<[&ToSql]>) -> Result<(), DatabaseError> {
//     let _rows = conn.query(sql, &params)?;
//     Ok(())
// }

// pub fn producing_hashmap<'a, T: Eq + Hash + FromSql>(conn: &Connection, sql: &str, params: Box<[&ToSql]>) -> Result<HashMap<T, i32>, DatabaseError> {
//     let rows = conn.query(sql, &params)?;
//     let mut hm = HashMap::new();
//     for row in rows.iter() {
//         hm.insert(row.get(0), row.get(1));
//     }
//     Ok(hm)
// }
