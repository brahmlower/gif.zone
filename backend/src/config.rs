// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
use config_::Config;
use config_::File;
// -----------------------------------------------------------------------------
use models::Environment;
// -----------------------------------------------------------------------------

pub fn get_config(path: &str) -> AppConfig {
    let mut settings = Config::default();
    settings.merge(File::with_name(path)).unwrap();
    let app_config = AppConfig::from(&settings);
    app_config
}

#[derive(Debug)]
pub struct HttpConfig {
    host: String,
    port: i64,
}

impl<'a> From<&'a Config> for HttpConfig {
    fn from(config: &Config) -> Self {
        HttpConfig {
            host: config.get_str("app.host").unwrap(),
            port: config.get_int("app.port").unwrap(),
        }
    }
}

impl ToString for HttpConfig {
    fn to_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug)]
pub struct DatabaseConfig {
    host: String,
    port: i64,
    user: String,
    pass: String,
}

impl<'a> From<&'a Config> for DatabaseConfig {
    fn from(config: &Config) -> Self {
        DatabaseConfig {
            host: config.get_str("database.host").unwrap(),
            port: config.get_int("database.port").unwrap(),
            user: config.get_str("database.user").unwrap(),
            pass: config.get_str("database.pass").unwrap(),
        }
    }
}

impl ToString for DatabaseConfig {
    fn to_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.user, self.pass, self.host, self.port
        )
    }
}

#[derive(Debug)]
pub struct AppConfig {
    pub environment: Environment,
    pub http: HttpConfig,
    pub database: DatabaseConfig,
}

impl<'a> From<&'a Config> for AppConfig {
    fn from(config: &Config) -> Self {
        let config_env = config.get_str("environment").unwrap();
        let env = config_env.parse::<Environment>().unwrap();
        AppConfig {
            environment: env,
            http: HttpConfig::from(config),
            database: DatabaseConfig::from(config),
        }
    }
}
