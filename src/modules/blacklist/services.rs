use actix_web::{get, patch, post, HttpResponse};
use actix_web::web::ServiceConfig;
use the_logger::TheLogger;

pub fn blacklist_services(cfg: &mut ServiceConfig) {
    cfg.service(get_open_blacklist)
        .service(get_client_status)
        .service(post_to_blacklist)
        .service(patch_blacklist_entry);
}

#[get("")]
async fn get_open_blacklist() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[get("{clients_id}")]
async fn get_client_status() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}
#[post("")]
async fn post_to_blacklist() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[patch("{blacklist_id}")]
async fn patch_blacklist_entry() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}