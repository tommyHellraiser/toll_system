use actix_web::{get, patch, post, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::web::ServiceConfig;
use the_logger::TheLogger;
use crate::utilities::functions::json_response;

pub fn blacklist_services(cfg: &mut ServiceConfig) {
    cfg.service(get_open_blacklist)
        .service(get_client_status)
        .service(post_to_blacklist)
        .service(patch_blacklist_entry);
}

/// GET {base_path}/api/blacklist
#[get("")]
async fn get_open_blacklist() -> HttpResponse {

    let logger = TheLogger::instance();

    json_response("open_blacklist response", StatusCode::OK)
}

/// GET {base_path}/api/blacklist/{clients_ID}
#[get("{clients_id}")]
async fn get_client_status() -> HttpResponse {

    let logger = TheLogger::instance();

    json_response("client_status response", StatusCode::OK)
}

/// POST {base_path}/api/blacklist
#[post("")]
async fn post_to_blacklist() -> HttpResponse {

    let logger = TheLogger::instance();

    json_response("post_to_blacklist response", StatusCode::OK)
}

/// PATCH {base_path}/api/blacklist/{blacklist_ID}
#[patch("{blacklist_id}")]
async fn patch_blacklist_entry() -> HttpResponse {

    let logger = TheLogger::instance();

    json_response("patch_blacklist_entry response", StatusCode::OK)
}