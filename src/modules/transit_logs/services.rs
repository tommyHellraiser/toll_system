use actix_web::{get, HttpResponse};
use actix_web::web::ServiceConfig;
use the_logger::TheLogger;

pub fn transit_logs_services(cfg: &mut ServiceConfig) {
    cfg.service(get_transit_log_by_license_plate)
        .service(get_transit_log_by_client_id);
}

#[get("{license_plate}")]
async fn get_transit_log_by_license_plate() -> HttpResponse {
    
    let logger = TheLogger::instance();
    
    HttpResponse::Ok().finish()
}

#[get("{clients_id}")]
async fn get_transit_log_by_client_id() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}
