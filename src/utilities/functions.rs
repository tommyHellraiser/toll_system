use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder};
use serde::Serialize;

pub fn json_response<C: Serialize>(content: C, status_code: StatusCode) -> HttpResponse {
    
    HttpResponseBuilder::new(status_code).json(content)
}
