use the_logger::{log_error, log_info};

mod config;
mod api;
mod modules;

#[tokio::main]
async fn main() {
    
    let logger = the_logger::TheLogger::instance();
    
    match api::start_api().await {
        Ok(_) => {
            log_info!(logger, "Api stopped at: {}", chrono::Local::now());
        },
        Err(e) => {
            log_error!(logger, "Failed to start Api: {}", e.to_string())
        }
    }   
}
