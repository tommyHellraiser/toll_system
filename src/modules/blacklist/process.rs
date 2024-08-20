use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use mysql_async::Conn;
use the_logger::TheLogger;
use crate::modules::blacklist::Blacklist;
use crate::modules::blacklist::requests::BlacklistPost;
use crate::modules::clients::Clients;
use crate::result_or_internal_error_for_api;
use crate::utilities::functions::json_response;
use the_logger::{log_error};
use error_mapper::traceback;

pub(super) async fn process_new_blacklist_entry(
    conn: &mut Conn,
    logger: &TheLogger,
    mut post_request: BlacklistPost
) -> HttpResponse {
    //  Validate fields before proceeding
    let errors = post_request.validate();
    if !errors.is_empty() {
        return json_response(errors.join(","), StatusCode::BAD_REQUEST)
    }

    //  Check if a client or license plate is already blacklisted with an open entry
    let available = result_or_internal_error_for_api!(
        Blacklist::check_available_to_insert(conn, post_request.clone()).await,
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
            Clients::select_by_license_plate(conn, &post_request.license_plate).await,
            logger
        );

        //  If a client exists, save it into the post_request
        if let Some(client) = client {
            post_request.clients_id = Some(client.get_id());
        }
    }

    let entry = result_or_internal_error_for_api!(
        Blacklist::create_new_entry(conn, post_request).await,
        logger
    );

    if let Some(entry) = entry {
        return json_response(entry, StatusCode::OK)
    }

    json_response("Request was not processed. Contact administration", StatusCode::NOT_MODIFIED)
}
