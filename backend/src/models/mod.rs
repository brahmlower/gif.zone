
pub mod account;
pub mod error;
pub mod gif;
pub mod tag;
pub mod search;

pub trait UriParam {
    fn as_uri_param() -> &'static str where Self: Sized;
}
