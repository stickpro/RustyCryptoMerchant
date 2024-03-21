use futures::FutureExt;
use rust_merchant::{configure, util};
use rust_merchant::constant::CONFIG;
use tracing::info;
use rust_merchant::error::AppResult;
use rust_merchant::server::AppServer;
use rust_merchant::server::worker::MessengerTask;

#[tokio::main]
async fn main() -> AppResult<()> {
    let _file_appender_guard = configure::tracing::init()?;
    info!("The initialization of Tracing was successful.");
    let config = CONFIG.clone();
    info!("Reading the config file was successful.");
    info!("Create a new server.");
    let server = AppServer::new(config).await?;
    info!("Create a new messenger task.");
    let messenger = MessengerTask::new(server.state.clone());
    info!("Run the server.");
    util::task::join_all(vec![
        (true, server.run().boxed()),
        (true, messenger.run().boxed()),
    ])
        .await?;
    Ok(())
}

