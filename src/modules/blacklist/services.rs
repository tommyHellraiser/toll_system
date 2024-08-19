use actix_web::{get, patch, post, web, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::web::ServiceConfig;
use error_mapper::{traceback};
use the_logger::{log_error, TheLogger};
use crate::{get_connection_for_api, result_or_internal_error_for_api};
use crate::modules::blacklist::Blacklist;
use crate::modules::blacklist::requests::{BlacklistPath, BlacklistPost, ClientPath};
use crate::modules::clients::Clients;
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
    let mut conn = get_connection_for_api!(logger);
    
    let content = result_or_internal_error_for_api!(
        Blacklist::select_open_blacklist(&mut conn).await,
        logger
    );
    
    json_response(content, StatusCode::OK)
}

/// GET {base_path}/api/blacklist/{clients_ID}
#[get("{clients_id}")]
async fn get_client_status(path: web::Path<ClientPath>) -> HttpResponse {

    let logger = TheLogger::instance();
    let mut conn = get_connection_for_api!(logger);
    
    let content = result_or_internal_error_for_api!(
        Blacklist::select_by_clients_id(&mut conn, logger, path.clients_id).await,
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

/// POST {base_path}/api/blacklist
#[post("")]
async fn post_to_blacklist(body: web::Json<BlacklistPost>) -> HttpResponse {

    let logger = TheLogger::instance();
    let mut conn = get_connection_for_api!(logger);
    let mut post_request = body.into_inner();
    
    //  Validate fields before proceeding
    let errors = post_request.validate();
    if !errors.is_empty() {
        return json_response(errors.join(","), StatusCode::BAD_REQUEST)
    }
    
    //  Check if a client or license plate is already blacklisted with an open entry
    let available = result_or_internal_error_for_api!(
        Blacklist::check_available_to_insert(&mut conn, post_request.clone()).await,
        logger
    );
    
    if !available {
        return json_response(
            format!("License plate {} is already blacklisted", post_request.license_plate),
            StatusCode::BAD_REQUEST
        )
    }
    
    //  If there's no client information in request, look for it
    if post_request.clients_id.is_none() {
        //  Get client from license plate if exists
        let client = result_or_internal_error_for_api!(
            Clients::select_by_license_plate(&mut conn, &post_request.license_plate).await,
            logger
        );

        //  If a client exists, save it into the post_request
        if let Some(client) = client {
            post_request.clients_id = Some(client.get_id());
        }
    }    
    
    let entry = result_or_internal_error_for_api!(
        Blacklist::create_new_entry(&mut conn, post_request).await,
        logger
    );
    
    if let Some(entry) = entry {
        return json_response(entry, StatusCode::OK)
    }
    
    json_response("Request was not processed. Contact administration", StatusCode::NOT_MODIFIED)
}

/// PATCH {base_path}/api/blacklist/{blacklist_ID}
#[patch("{blacklist_id}")]
async fn patch_blacklist_entry(path: web::Path<BlacklistPath>) -> HttpResponse {

    let logger = TheLogger::instance();

    json_response("patch_blacklist_entry response", StatusCode::OK)
}