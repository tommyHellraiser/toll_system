use actix_web::{get, patch, post, web, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::web::ServiceConfig;
use error_mapper::{traceback};
use the_logger::{log_error, TheLogger};
use crate::{get_connection_for_api, result_or_internal_error_for_api};
use crate::modules::blacklist;
use crate::modules::blacklist::Blacklist;
use crate::modules::blacklist::requests::{BlacklistLicensePlatePath, BlacklistPatch, BlacklistPath, BlacklistPost, ClientIdPath};
use crate::utilities::functions::json_response;

pub fn blacklist_services(cfg: &mut ServiceConfig) {
    cfg.service(get_open_blacklist)
        .service(get_client_status_by_id)
        .service(get_client_status_by_license_plate)
        .service(post_to_blacklist)
        .service(patch_blacklist_entry);
}

/// GET {base_path}/api/blacklist
#[get("")]
async fn get_open_blacklist() -> HttpResponse {

    let logger = TheLogger::instance();
    let mut conn = get_connection_for_api!(logger);
    
    let content = result_or_internal_error_for_api!(
        Blacklist::select_open_blacklist(&mut conn).await,
        logger
    );
    
    json_response(content, StatusCode::OK)
}

/// GET {base_path}/api/blacklist/clients/{clients_ID}
#[get("clients/{clients_id}")]
async fn get_client_status_by_id(path: web::Path<ClientIdPath>) -> HttpResponse {

    let logger = TheLogger::instance();
    let mut conn = get_connection_for_api!(logger);
    
    let content = result_or_internal_error_for_api!(
        Blacklist::select_by_clients_id(&mut conn, path.clients_id).await,
        logger
    );
    
    if let Some(blacklisted_client) = content {
        return json_response(blacklisted_client, StatusCode::OK)
    }

    json_response(
        format!("Client ID: {} is not blacklisted", path.clients_id), 
        StatusCode::NOT_FOUND
    )
}

/// GET {base_path}/api/blacklist/license_plates/{license_plate}
#[get("license_plates/{license_plate}")]
async fn get_client_status_by_license_plate(path: web::Path<BlacklistLicensePlatePath>) -> HttpResponse {
    
    let logger = TheLogger::instance();
    let mut conn = get_connection_for_api!(logger);
    
    let content = result_or_internal_error_for_api!(
        Blacklist::select_by_license_plate(&mut conn, &path.license_plate).await,
        logger
    );
    
    if let Some(blacklist) = content {
        return json_response(blacklist, StatusCode::OK)
    }
    
    HttpResponse::NotFound().finish()
}

/// POST {base_path}/api/blacklist
#[post("")]
async fn post_to_blacklist(body: web::Json<BlacklistPost>) -> HttpResponse {

    let logger = TheLogger::instance();
    let mut conn = get_connection_for_api!(logger);
    let post_request = body.into_inner();
    
    blacklist::process::process_new_blacklist_entry(&mut conn, logger, post_request).await
}

/// PATCH {base_path}/api/blacklist/{blacklist_ID}
#[patch("{blacklist_id}")]
async fn patch_blacklist_entry(
    path: web::Path<BlacklistPath>,
    body: web::Json<BlacklistPatch>
) -> HttpResponse {

    let logger = TheLogger::instance();
    let mut conn = get_connection_for_api!(logger);
    let id = path.blacklist_id;
    let patch_request = body.into_inner();
    
    let errors = patch_request.validate();
    if !errors.is_empty() {
        return json_response(errors.join(","), StatusCode::BAD_REQUEST)
    }

    //  Return if patch request is empty
    if patch_request.reason.is_none() && patch_request.restriction_expiry.is_none() {
        return HttpResponse::NotModified().finish()
    }
    
    let result = result_or_internal_error_for_api!(
        Blacklist::update_blacklist_entry(&mut conn, id, patch_request).await,
        logger
    );
    
    if !result {
        return json_response(format!("Blacklist entry ID: {} not modified", id), StatusCode::NOT_MODIFIED)
    }
    
    HttpResponse::Ok().finish()
}