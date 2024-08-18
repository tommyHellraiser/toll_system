use actix_web::{get, patch, post, HttpResponse};
use actix_web::web::ServiceConfig;
use the_logger::TheLogger;

pub fn violation_logs_services(cfg: &mut ServiceConfig) {
    cfg.service(get_violation_log_by_license_plate)
        .service(get_violation_log_by_license_plate)
        .service(post_violation_log)
        .service(patch_violation_log);
}

#[get("{clients_id}")]
async fn get_violation_log_by_client_id() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[get("{license_plate}")]
async fn get_violation_log_by_license_plate() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[post("")]
async fn post_violation_log() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[patch("{violation_logs_id}")]
async fn patch_violation_log() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}
