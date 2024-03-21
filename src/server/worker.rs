use tracing::info;
use crate::error::AppResult;
use crate::server::state::AppState;

pub struct MessengerTask {
    state: AppState,
}

impl MessengerTask {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn run(self) -> AppResult {
        info!("The messenger task has started");
        Ok(())
    }
}