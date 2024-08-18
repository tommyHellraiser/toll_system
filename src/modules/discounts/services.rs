use actix_web::{get, patch, post, HttpResponse};
use actix_web::web::ServiceConfig;
use the_logger::TheLogger;

pub fn discount_services(cfg: &mut ServiceConfig) {
    cfg.service(get_discounts)
        .service(post_discount)
        .service(patch_discount);
}

#[get("")]
async fn get_discounts() -> HttpResponse {

    //  TODO get discounts only for current booth
    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[post("")]
async fn post_discount() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[patch("{discounts_id}")]
async fn patch_discount() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}