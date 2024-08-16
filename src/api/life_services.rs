use actix_web::{get, put, web, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::web::ServiceConfig;
use the_logger::{log_error, TheLogger};
use crate::api::ApiData;
use crate::{utilities, SERVICE_NAME, SERVICE_VERSION};

pub(super) fn life_services(cfg: &mut ServiceConfig) {
    cfg.service(alive).service(stop);
}

#[get("/alive")]
async fn alive() -> HttpResponse {

    let response = format!(
        "{} alive at {}, version: {}",
        SERVICE_NAME,
        chrono::Local::now().format("%d-%m-%Y %H:%M:%S"),
        SERVICE_VERSION
    );

    utilities::functions::json_response(response, StatusCode::OK)
}

#[put("/stop")]
async fn stop(api_data: web::Data<ApiData>) -> HttpResponse {

    if let Err(e) = api_data.api_stopper.send(true).await {
        log_error!(TheLogger::instance(), "Error sending stop signal: {}", e);
        return HttpResponse::InternalServerError().finish()
    };

    utilities::functions::json_response("Stopping service...", StatusCode::OK)
}
