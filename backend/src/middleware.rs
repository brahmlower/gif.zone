
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use iron::Request;
use iron::IronResult;
use iron::typemap::Key;
use iron::BeforeMiddleware;
use r2d2;
use r2d2_postgres::TlsMode;
use r2d2_postgres::PostgresConnectionManager;
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------

// Heavily pulled from https://github.com/DavidBM/rust-webserver-example-with-iron-diesel-r2d2-serde/blob/master/src/http_adaptor/adaptor.rs

pub type PostgresConnection = r2d2::PooledConnection<PostgresConnectionManager>;
pub type PostgresPool = r2d2::Pool<PostgresConnectionManager>;


pub struct PostgresMiddleware {
    pool: PostgresPool
}

impl PostgresMiddleware {
    pub fn new(conn_str: String) -> PostgresMiddleware {
        let res_conman = PostgresConnectionManager::new(conn_str, TlsMode::None);
        let manager = match res_conman {
            Err(e) => panic!("Connection manager failed: {}", e),
            Ok(m)  => m
        };
        let res_pool = r2d2::Pool::new(manager);
        let pool = match res_pool {
            Err(e) => panic!("Failure while creating pool: {}", e),
            Ok(p)  => p
        };
        PostgresMiddleware { pool: pool }
    }
}

pub struct Value(PostgresPool);

impl Key for PostgresMiddleware {
    type Value = Value;
}

impl BeforeMiddleware for PostgresMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<PostgresMiddleware>(Value(self.pool.clone()));
        Ok(())
    }
}

pub trait PostgresReqExt {
    fn get_db_conn(&self) -> PostgresConnection;
}

impl <'a, 'b>PostgresReqExt for Request<'a, 'b> {
    fn get_db_conn(&self) -> PostgresConnection {
        let &Value(ref pool) = self.extensions.get::<PostgresMiddleware>().unwrap();
        return pool.get().expect("Failed to get a db connection");
    }
}
