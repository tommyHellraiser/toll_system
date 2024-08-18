use actix_web::{post, HttpResponse};
use actix_web::web::ServiceConfig;
use the_logger::TheLogger;

pub fn transactions_services(cfg: &mut ServiceConfig) {
    cfg.service(new_transaction);
}

#[post("")]
async fn new_transaction() -> HttpResponse {
    
    let logger = TheLogger::instance();
    
    HttpResponse::Ok().finish()
}
