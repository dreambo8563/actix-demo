use config::{Config, ConfigError, Environment, File};
use log::debug;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    echo: bool,
}

#[derive(Debug, Deserialize)]
pub struct Sparkpost {
    key: String,
    token: String,
    url: String,
    version: u8,
}

#[derive(Debug, Deserialize)]
pub struct Twitter {
    consumer_token: String,
    consumer_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Braintree {
    merchant_id: String,
    public_key: String,
    private_key: String,
}
#[derive(Debug, Deserialize)]
pub struct LogConfig {
    format: String,
    source_location: String,
    timezone: String,
    pub level: String,
    pub path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub sparkpost: Sparkpost,
    pub twitter: Twitter,
    pub braintree: Braintree,
    pub logconfig: LogConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        debug!("env:{}", env);
        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .merge(File::with_name(&format!("config/{}", env)).required(false))?
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .merge(File::with_name("config/local").required(false))?
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .merge(Environment::new())?
            // You may also programmatically change settings
            .set("database.url", "postgres://")?;
        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
