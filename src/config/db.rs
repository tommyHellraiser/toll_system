use std::fs::File;
use std::io::Read;
use error_mapper::{create_new_error, TheResult};
use lazy_static::lazy_static;
use mysql_async::{Conn, Pool};
use mysql_async::prelude::Queryable;
use tokio::sync::RwLock;

pub const SCHEMA_RESET_SCRIPT: &str = "config/schema_reset.sql";

lazy_static! {
    static ref DB_CONN: DbConn = DbConn::new();
}

pub struct DbConn {
    inner: RwLock<DbConnInner>
}

struct DbConnInner {
    pool: Option<Pool>
}

impl DbConn {
    fn new() -> Self {
        Self {
            inner: RwLock::new(DbConnInner { pool: None })
        }
    }
    
    fn instance() -> &'static Self {
        &DB_CONN
    }
    
    pub async fn configure_pool(pool: Pool) {
        Self::instance().inner.write().await.pool = Some(pool);
    }
    
    pub async fn get_pool() -> Option<Pool> {
        Self::instance().inner.read().await.pool.clone()
    }
}

pub async fn get_connection() -> TheResult<Conn> {
    let pool = DbConn::get_pool()
        .await
        .ok_or(create_new_error!("Couldn't get connection pool for database"))?;
    
    let conn = pool.get_conn().await.map_err(|e| create_new_error!(e))?;
    
    Ok(conn)
}

pub async fn reset_schema(conn: &mut Conn) -> TheResult<()> {

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
    
    conn.query_drop(schema_reset_script).await.map_err(|e| create_new_error!(e))
}
