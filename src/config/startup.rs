use std::fs::File;
use std::io::Read;
use error_mapper::{create_new_error, TheResult};
use mysql_async::prelude::Queryable;
use crate::config::db;
use crate::config::db::DbConn;
use crate::config::environment::Environment;

const SCHEMA_RESET_SCRIPT: &str = "config/schema_reset.sql";

/// ## Description
/// Executes initial setup for the app
pub async fn startup_configurations() ->TheResult<()> {
    
    //  Create the initial pool of database connections
    let pool = mysql_async::Pool::new(
        Environment::get_database_config().await.get_address().as_str()
    );
    
    //  Save the pool of connections in a singleton static reference for global access
    DbConn::configure_pool(pool).await;
    
    if !Environment::get_database_config().await.get_reset_schema() {
        return Ok(())
    }
    
    //  Open schema reset script
    let mut reader = File::options()
        .read(true)
        .write(false)
        .create(false)
        .append(false)
        .open(SCHEMA_RESET_SCRIPT)
        .map_err(|e| create_new_error!(e))?;
    let mut schema_reset_script = String::new();
    reader.read_to_string(&mut schema_reset_script).map_err(|e| create_new_error!(e))?;
    
    //  Run the schema reset script
    let mut conn = db::get_connection().await?;
    conn.query_drop(schema_reset_script).await.map_err(|e| create_new_error!(e))?;

    Ok(())
}