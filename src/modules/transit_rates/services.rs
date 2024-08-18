use actix_web::{get, patch, post, HttpResponse};
use actix_web::web::ServiceConfig;
use the_logger::TheLogger;


pub fn transit_rates_services(cfg: &mut ServiceConfig) {
    cfg.service(get_transit_rates)
        .service(get_transit_rate)
        .service(post_transit_rate)
        .service(patch_transit_rate);
}

#[get("")]
async fn get_transit_rates() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[get("/{transit_rates_id}")]
async fn get_transit_rate() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[post("")]
async fn post_transit_rate() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[patch("{transit_rates_id}")]
async fn patch_transit_rate() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}
