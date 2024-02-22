use actix_web::{HttpResponse, web, http::StatusCode, Responder};
use stayalive_api::state::AppState;
use actix_web::http::header;
use crate::modules::auth::controller::{Auth, AuthController};
use super::structs::LoginPayload;
use crate::modules::auth::controller::Jwt;


impl Auth{
    pub async fn login(payload: web::Json<LoginPayload>, app_state: web::Data<AppState>) -> impl Responder {
        let LoginPayload {email, password} = payload.into_inner();
        let conn = &app_state.db_connection;

        let (is_valid, user_id): (bool, String) = match &AuthController.check_login(email, password, conn).await {
            Ok(is_valid) => is_valid.to_owned(),
            Err(status_code) => return HttpResponse::build(StatusCode::from_u16(*status_code).unwrap()),
        };

        match is_valid{
            true => {
                let mut response = HttpResponse::Ok();
                response.append_header((header::AUTHORIZATION, Jwt.create(user_id)));
                response 
            },
            false => HttpResponse::Forbidden()
        }
    }
}