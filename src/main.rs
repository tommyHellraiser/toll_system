use mysql_async::prelude::Queryable;
use the_logger::{log_error, log_info};
use crate::config::environment::Environment;
use crate::config::startup;

mod config;
mod api;
mod modules;
mod utilities;

const SERVICE_NAME: &str = env!("CARGO_PKG_NAME");
const SERVICE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    
    let logger = the_logger::TheLogger::instance();

    if let Err(e) = startup::startup_configurations().await {
        log_error!(logger, "Failed to execute initial setup: {}", e)
    }
    
    let pool = mysql_async::Pool::new(
        Environment::get_database_config().await.get_address().as_str()
    );
    let mut conn = pool.get_conn().await.expect("Couldn't get connection");
    
    if Environment::get_database_config().await.get_reset_schema() {
        conn.query_drop("INSERT INTO test_insert_table (value) VALUES ('hola');").await.unwrap();
    }
    
    match api::start_api().await {
        Ok(_) => {
            log_info!(logger, "Api stopped at: {}", chrono::Local::now());
        },
        Err(e) => {
            log_error!(logger, "Failed to start Api: {}", e.to_string())
        }
    }   
}
