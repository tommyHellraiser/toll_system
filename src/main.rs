use the_logger::{log_error, log_info};
use crate::config::startup;

mod config;
mod api;
mod modules;
mod utilities;

const SERVICE_NAME: &str = env!("CARGO_PKG_NAME");
const SERVICE_VERSION: &str = env!("CARGO_PKG_VERSION");
const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[tokio::main]
async fn main() {
    
    let logger = the_logger::TheLogger::instance();

    if let Err(e) = startup::startup_configurations().await {
        log_error!(logger, "Failed to execute initial setup: {}", e)
    }
    
    match api::start_api().await {
        Ok(_) => {
            log_info!(
                logger, 
                "Api stopped at: {}",
                chrono::Local::now().format(DATETIME_FORMAT)
            );
        },
        Err(e) => {
            log_error!(logger, "Failed to start Api: {}", e.to_string())
        }
    }
}
