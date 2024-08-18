use std::any::Any;
use actix_web::{delete, get, patch, post, HttpResponse};
use actix_web::web::ServiceConfig;
use the_logger::TheLogger;

pub fn clients_services(cfg: &mut ServiceConfig) {
    cfg.service(get_client)
        .service(get_client_by_id)
        .service(post_client)
        .service(patch_client)
        .service(delete_client);
}

#[get("")]
async fn get_client() -> HttpResponse {

    //  TODO add search params
    let logger = TheLogger::instance();
    
    HttpResponse::Ok().finish()
}

#[get("{clients_id}")]
async fn get_client_by_id() -> HttpResponse {
    
    let logger = TheLogger::instance();
    
    HttpResponse::Ok().finish()
}

#[post("")]
async fn post_client() -> HttpResponse {
    
    let logger = TheLogger::instance();
    
    HttpResponse::Ok().finish()
}

#[patch("{clients_id}")]
async fn patch_client() -> HttpResponse {
    
    let logger = TheLogger::instance();
    
    HttpResponse::Ok().finish()
}

#[delete("{clients_id}")]
async fn delete_client() -> HttpResponse {
    
    let logger = TheLogger::instance();
    
    HttpResponse::Ok().finish()
}
