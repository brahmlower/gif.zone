
mod gif;
mod util;

// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use iron::AfterMiddleware;
use iron::BeforeMiddleware;
use iron::Chain;
use iron::headers;
use iron::Request;
use iron::Response;
use iron::IronResult;
use router::Router;
// -----------------------------------------------------------------------------
use middleware::PostgresMiddleware;
// -----------------------------------------------------------------------------

pub fn api(db_uri: String) -> Chain {
    let routes = routes();
    let mut chain = Chain::new(routes);
    chain.link_before(PostgresMiddleware::new(db_uri));
    chain.link_before(LogRequest);
    chain.link_after(DefaultContentType);
    chain
}

/// These routes are served publicly. They are restricted such that there is
/// limited write access to various endpoints, so that in most cases, change
/// may only be brought about via action submitions.
fn routes() -> Router {
    let mut router = Router::new();

    // Gif endpoints
    router.get(     "/gif",         gif::list,      "gif_list");
    router.get(     "/gif/:gif",    gif::get,       "gif_get");
    router.get(     "/search",      gif::search,    "gif_search");

    router
}

struct DefaultContentType;

impl AfterMiddleware for DefaultContentType {
    // This is run for every requests, AFTER all handlers have been executed
    fn after(&self, _req: &mut Request, mut resp: Response) -> IronResult<Response> {
        if resp.headers.get::<headers::ContentType>() == None {
            // Set a standard header
            resp.headers.set(headers::ContentType::json());
        }
        Ok(resp)
    }
}

struct LogRequest;

impl BeforeMiddleware for LogRequest {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        info!("{} {} {}", req.method, req.url, req.remote_addr);
        Ok(())
    }
}
