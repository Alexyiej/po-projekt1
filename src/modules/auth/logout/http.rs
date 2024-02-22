use actix_web::{HttpResponse, Responder};
use crate::modules::auth::controller::Auth;


impl Auth{
    pub async fn logout() -> impl Responder {
        HttpResponse::Ok().body("OK")
    }
}