use actix_web::{get, put, web, HttpResponse};
use actix_web::web::ServiceConfig;
use the_logger::{log_error, TheLogger};
use crate::api::ApiData;

pub(super) fn life_services(cfg: &mut ServiceConfig) {
    cfg.service(alive).service(stop);
}

#[get("/alive")]
async fn alive() -> HttpResponse {
    
    HttpResponse::Ok().finish()
}

#[put("/stop")]
async fn stop(api_data: web::Data<ApiData>) -> HttpResponse {
    
    if let Err(e) = api_data.api_stopper.send(true).await {
        log_error!(TheLogger::instance(), "Error sending stop signal: {}", e);
        return HttpResponse::InternalServerError().finish()
    };
    
    HttpResponse::Ok().finish()
}
