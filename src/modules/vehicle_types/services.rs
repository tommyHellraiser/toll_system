use actix_web::{get, patch, post, HttpResponse};
use actix_web::web::ServiceConfig;
use the_logger::TheLogger;

pub fn vehicle_types_services(cfg: &mut ServiceConfig) {
    cfg.service(get_vehicle_types)
        .service(post_vehicle_type)
        .service(patch_vehicle_type);
}

#[get("")]
async fn get_vehicle_types() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}


#[post("")]
async fn post_vehicle_type() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}

#[patch("{vehicle_types_id}")]
async fn patch_vehicle_type() -> HttpResponse {

    let logger = TheLogger::instance();

    HttpResponse::Ok().finish()
}
