use std::fs::File;
use std::io::Read;
use error_mapper::{create_new_error, TheResult};
use mysql_async::prelude::Queryable;
use the_logger::{log_info, TheLogger};
use crate::config::db;
use crate::config::db::DbConn;
use crate::config::environment::Environment;
use crate::DATETIME_FORMAT;


/// ## Description
/// Executes initial setup for the app
pub async fn startup_configurations() ->TheResult<()> {
    
    let logger = TheLogger::instance();
    log_info!(
        logger,
        "Initializing startup configurations at: {}",
        chrono::Local::now().format(DATETIME_FORMAT)
    );
    
    //  Create the initial pool of database connections
    let pool = mysql_async::Pool::new(
        Environment::get_database_config().await.get_address().as_str()
    );
    
    //  Save the pool of connections in a singleton static reference for global access
    DbConn::configure_pool(pool).await;
    
    if !Environment::get_database_config().await.get_reset_schema() {
        return Ok(())
    }
        
    //  Run the schema reset script
    let mut conn = db::get_connection().await?;
    
    db::reset_schema(&mut conn).await?;
    
    log_info!(
        logger,
        "Completed startup configuration at: {}",
        chrono::Local::now().format(DATETIME_FORMAT)
    );

    Ok(())
}