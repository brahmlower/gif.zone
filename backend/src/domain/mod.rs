
pub mod gif;
pub mod tag;

use models::AppInfo;
use models::Environment;
use models::error::DomainError;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn app_info() -> Result<AppInfo, DomainError> {
    Ok(AppInfo {
        name: "backend",
        version: VERSION,
        environment: Environment::Test
    })
}
