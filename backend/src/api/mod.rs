
mod gif;
mod tag;
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
use domain;
// -----------------------------------------------------------------------------

pub fn api(db_uri: String) -> Chain {
    let routes = routes();
    info!("Defining route chain");
    let mut chain = Chain::new(routes);
    info!("Starting database middleware");
    chain.link_before(PostgresMiddleware::new(db_uri));
    chain.link_before(LogRequest);
    chain.link_after(DefaultContentType);
    chain
}

fn routes() -> Router {
    let mut router = Router::new();

    // Info endpoint
    router.get(     "/",    info,  "info");
    // Gif endpoints
    router.get(     "/gif",             gif::list,      "gif_list");
    router.get(     "/gif/:gif",        gif::get,       "gif_get");
    router.get(     "/gif/:gif/tags",   gif::get_tags,  "gif_get_tags");
    router.get(     "/search",          gif::search,    "gif_search");
    // Tag endpoints
    router.get(     "/tag",         tag::list,  "tag_list");
    router.get(     "/tag/:tag",    tag::get,   "tag_get");

    router
}

struct DefaultContentType;

/// All content returned by the API is application/json
impl AfterMiddleware for DefaultContentType {
    fn after(&self, _req: &mut Request, mut resp: Response) -> IronResult<Response> {
        if resp.headers.get::<headers::ContentType>() == None {
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

/// Gets all gifs. This should rarely be used I think
fn info(_req: &mut Request) -> IronResult<Response> {
    let result = domain::app_info();
    util::result_to_ironresult(result)
}
