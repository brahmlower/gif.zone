
// #![deny(unused_extern_crates)]

extern crate chrono;
extern crate jsonwebtoken as jwt;
extern crate serde;
extern crate serde_json;
extern crate bodyparser;
extern crate iron;
extern crate router;
extern crate config as config_;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate fern;
#[macro_use] extern crate postgres;
#[macro_use] extern crate postgres_derive;
extern crate r2d2;
extern crate r2d2_postgres;

// -----------------------------------------------------------------------------
use std::env;
// -----------------------------------------------------------------------------
use iron::prelude::*;
//------------------------------------------------------------------------------
use config::get_config;
use logging::setup_logger;
// -----------------------------------------------------------------------------

mod api;
mod database;
mod domain;
mod config;
mod logging;
mod middleware;
mod models;

fn main() {
    setup_logger().unwrap();
    let args: Vec<String> = env::args().collect();
    let config_path = match args.get(1) {
        Some(path)  => path,
        None        => "./settings.yml"
    };

    info!("Reading app config from: {}", config_path);
    let app_config  = get_config(config_path);
    let db_uri      = app_config.database.to_string();
    let http_str    = app_config.http.to_string();

    info!("Establishing routes and database middleware");
    let chain = api::api(db_uri);

    info!("Listening for requests at: {}", http_str);
    Iron::new(chain).http(http_str).unwrap();
}
