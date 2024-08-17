use error_mapper::{create_new_error, TheResult};
use lazy_static::lazy_static;
use mysql_async::{Conn, Pool};
use tokio::sync::RwLock;

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
