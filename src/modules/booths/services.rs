use actix_web::{delete, get, patch, post, HttpResponse};
use actix_web::web::ServiceConfig;
use the_logger::TheLogger;

pub fn booths_services(cfg: &mut ServiceConfig) {
    cfg.service(get_booths)
        .service(get_booth);
}

pub fn booths_internal_services(cfg: &mut ServiceConfig) {
    cfg.service(post_booth)
        .service(patch_booth)
        .service(delete_booth);
}

#[get("")]
async fn get_booths() -> HttpResponse {
    
    let logger = TheLogger::instance();
    
    HttpResponse::Ok().finish()
}

#[get("{booths_id}")]
async fn get_booth() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[post("")]
async fn post_booth() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[patch("{booths_id}")]
async fn patch_booth() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[delete("{booths_id}")]
async fn delete_booth() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}
