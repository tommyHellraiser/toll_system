use actix_web::{post, HttpResponse};
use actix_web::web::ServiceConfig;
use the_logger::TheLogger;

pub fn transit_services(cfg: &mut ServiceConfig) {
    cfg.service(new_incoming_transit)
        .service(post_violation_log);
}

#[post("")]
async fn new_incoming_transit() -> HttpResponse {
    
    let logger = TheLogger::instance();
    
    HttpResponse::Ok().finish()
}

#[post("")]
async fn post_violation_log() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}
