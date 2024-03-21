use std::str::FromStr;
use config::{ConfigError, Environment};
use serde::Deserialize;
use ::tracing::info;

use crate::util::dir::get_project_root;

use self::{db::DatabaseConfig, server::ServerConfig};

pub mod db;
pub mod tracing;
pub mod env;
pub mod server;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub profile: Profile,
    pub server: ServerConfig,
    pub db: DatabaseConfig,
}

impl AppConfig {
    pub fn read(env_src: Environment) -> Result<Self, config::ConfigError> {
        let config_dir = get_setting_dir()?;
        let profile = std::env::var("APP_PROFILE")
            .map(|env| Profile::from_str(&env).map_err(|e| ConfigError::Message(e.to_string())))
            .unwrap_or_else(|_e| Ok(Profile::Dev))?;

        let profile_filename = format!("{profile}.toml");
        let config = config::Config::builder()
            .add_source(config::File::from(config_dir.join("base.toml")))
            .add_source(config::File::from(config_dir.join(profile_filename)))
            .add_source(env_src)
            .build()?;
        info!("Successfully read config profile: {profile}.");
        config.try_deserialize()
    }
}

pub fn get_setting_dir() -> Result<std::path::PathBuf, ConfigError> {
    Ok(
        get_project_root()
            .map_err(|e| ConfigError::Message(e.to_string()))?
            .join("settings"),
    )
}

#[derive(
Debug, strum::Display, strum::EnumString, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy,
)]
pub enum Profile {
    #[serde(rename = "test")]
    #[strum(serialize = "test")]
    Test,
    #[serde(rename = "dev")]
    #[strum(serialize = "dev")]
    Dev,
    #[serde(rename = "prod")]
    #[strum(serialize = "prod")]
    Prod,
}