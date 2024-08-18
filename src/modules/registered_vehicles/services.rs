use actix_web::{get, patch, post, HttpResponse};
use actix_web::web::ServiceConfig;
use the_logger::TheLogger;

pub fn registered_vehicles_services(cfg: &mut ServiceConfig) {
    cfg.service(get_vehicles_by_client)
        .service(get_vehicle_by_license_plate)
        .service(post_vehicle)
        .service(patch_vehicle);
}

#[get("/{clients_id}")]
async fn get_vehicles_by_client() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[get("/{license_plate}")]
async fn get_vehicle_by_license_plate() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[post("")]
async fn post_vehicle() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[patch("{registered_vehicles_id}")]
async fn patch_vehicle() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}
