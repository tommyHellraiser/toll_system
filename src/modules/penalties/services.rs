use actix_web::{get, patch, post, HttpResponse};
use actix_web::web::ServiceConfig;
use the_logger::TheLogger;

pub fn penalties_services(cfg: &mut ServiceConfig) {
    cfg.service(get_open_penalties)
        .service(get_penalty_by_client)
        .service(post_penalty)
        .service(patch_penalty);
}

#[get("")]
async fn get_open_penalties() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[get("{clients_id}")]
async fn get_penalty_by_client() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[post("")]
async fn post_penalty() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[patch("{penalties_id}")]
async fn patch_penalty() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}