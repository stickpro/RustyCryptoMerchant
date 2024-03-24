use crate::{configure::AppConfig, error::AppResult};

pub mod database;


pub trait ClientBuilder: Sized {
    fn build_from_config(config: &AppConfig) -> AppResult<Self>;
}