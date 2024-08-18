use actix_web::{get, patch, HttpResponse};
use actix_web::web::ServiceConfig;
use the_logger::TheLogger;

pub fn client_balances_services(cfg: &mut ServiceConfig) {
    cfg.service(get_balance_by_client_id)
        .service(update_balance);
}

#[get("{clients_id}")]
async fn get_balance_by_client_id() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[patch("{clients_id}")]
async fn update_balance() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}
