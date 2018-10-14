use database::gif;
use database::tag;
use models::gif::GifId;
use models::tag::TagId;
use postgres::Connection;
use postgres::TlsMode;

const DB_STRING: &str = "postgres://gif_zone:gif_zone@localhost:5432";

fn get_db_conn() -> Connection {
    Connection::connect(DB_STRING, TlsMode::None).unwrap()
}

#[test]
fn gif_fetch_all() {
    let conn = get_db_conn();
    let result = gif::fetch_all(&conn);
    println!("{:?}", result);
    assert!(result.is_ok());
}

#[test]
fn gif_fetch_one() {
    let conn = get_db_conn();
    let id = GifId(1);
    let result = gif::fetch_one(&conn, &id);
    println!("{:?}", result);
    assert!(result.is_ok());
}

#[test]
fn tag_fetch_all() {
    let conn = get_db_conn();
    let result = tag::fetch_all(&conn);
    println!("{:?}", result);
    assert!(result.is_ok());
}

#[test]
fn tag_fetch_one() {
    let conn = get_db_conn();
    let id = TagId(1);
    let result = tag::fetch_one(&conn, &id);
    println!("{:?}", result);
    assert!(result.is_ok());
}

#[test]
fn tag_fetch_by_gif() {
    let conn = get_db_conn();
    let id = GifId(1);
    let result = tag::fetch_by_gif(&conn, &id);
    println!("{:?}", result);
    assert!(result.is_ok());
}
