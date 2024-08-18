use actix_web::{get, put, web, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::web::ServiceConfig;
use the_logger::{log_error, log_info, TheLogger};
use crate::api::ApiData;
use crate::{utilities, DATETIME_FORMAT, SERVICE_NAME, SERVICE_VERSION};
use crate::config::db;

pub(super) fn life_services(cfg: &mut ServiceConfig) {
    cfg.service(alive).service(stop).service(reset_schema);
}

#[get("/alive")]
async fn alive() -> HttpResponse {

    let response = format!(
        "{} alive at {}, version: {}",
        SERVICE_NAME,
        chrono::Local::now().format(DATETIME_FORMAT),
        SERVICE_VERSION
    );

    utilities::functions::json_response(response, StatusCode::OK)
}

#[put("/stop")]
async fn stop(api_data: web::Data<ApiData>) -> HttpResponse {

    let logger = TheLogger::instance();
    
    if let Err(e) = api_data.api_stopper.send(true).await {
        log_error!(logger, "Error sending stop signal: {}", e);
        return HttpResponse::InternalServerError().finish()
    };
    
    log_info!(logger, "Received stop signal by request");

    utilities::functions::json_response("Stopping service...", StatusCode::OK)
}

#[put("/reset_schema")]
async fn reset_schema() -> HttpResponse {
    
    let logger = TheLogger::instance();
    
    let Ok(mut conn) = db::get_connection().await else {
        return HttpResponse::InternalServerError().finish()
    };
    
    if let Err(e) = db::reset_schema(&mut conn).await {
        log_error!(logger, "Failed to reset database schema: {}", e);
        return HttpResponse::InternalServerError().finish()
    }
    
    log_info!(logger, "Schema reset executed successfully!");
    
    HttpResponse::Ok().finish()
}
