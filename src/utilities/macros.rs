#[macro_export]
macro_rules! panic_from_row {
    ($column: literal, $table: expr) => {
        std::panic::panic_any(
            format!("Coudln't find column name {} in table {}", $column, $table)
        )
    };
}

#[macro_export]
macro_rules! get_value_from_row {
    ($row: expr, $column: literal, $table: expr, $datatype: ty) => {
        if let Some(content) = $row.get::<$datatype, _>($column) {
            content
        } else {
            $crate::panic_from_row!($column, $table)
        }
    };
}

#[macro_export]
macro_rules! get_connection_for_api {
    ($logger: expr) => {
        match $crate::config::db::get_connection().await {
            Ok(connection) => connection,
            Err(error) => {
                the_logger::log_error!(
                    $logger,
                    "Coudln't get database connection from pool: {}",
                    error
                );
                return actix_web::HttpResponse::InternalServerError().finish()
            }
        }
    }
}

#[macro_export]
macro_rules! result_or_internal_error_for_api {
    ($function: expr, $logger: expr) => {
        match $function {
            Ok(content) => content,
            Err(error) => {
                let new_error = traceback!(error, "get_open_blacklist service");
                log_error!($logger, "Failed to execute request: {}", new_error);
                return HttpResponse::InternalServerError().finish()
            }
        }
    };
}
