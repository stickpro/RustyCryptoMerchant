use std::sync::Arc;
use tokio::sync::Notify;
use crate::configure::AppConfig;
use crate::error::AppResult;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub messenger_notify: Arc<Notify>
}

impl AppState {
    pub async fn new(config: AppConfig) -> AppResult<Self> {
        Ok(Self {
            config: Arc::new(config),
            messenger_notify: Default::default(),
        })
    }
}