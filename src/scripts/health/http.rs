use actix_web::{HttpResponse, Responder};
use crate::Health;


impl Health{
    pub async fn health() -> impl Responder {
        HttpResponse::Ok().body("OK")
    }
}
