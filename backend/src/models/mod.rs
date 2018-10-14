use std::str::FromStr;

pub mod account;
pub mod error;
pub mod gif;
pub mod search;
pub mod tag;

pub trait UriParam {
    fn as_uri_param() -> &'static str
    where
        Self: Sized;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Local,
    Test,
    Prod,
}

impl FromStr for Environment {
    type Err = ();
    fn from_str(s: &str) -> Result<Environment, ()> {
        match s {
            "local" => Ok(Environment::Local),
            "test" => Ok(Environment::Test),
            "prod" => Ok(Environment::Prod),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub environment: Environment,
}
