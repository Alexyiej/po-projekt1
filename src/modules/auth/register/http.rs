use actix_web::{HttpResponse, web, Responder};
use actix_web::http::header;
use validator::Validate;
use stayalive_api::state::AppState;
use stayalive_api::utils::constants::PASSWORD_STRENGTH;
use bcrypt::hash;
use crate::modules::auth::controller::Auth;
use super::structs::{RegisterPayload, User};
use crate::modules::auth::controller::Jwt;
use crate::modules::auth::db::AuthDatabaseController;


impl Auth{
    pub async fn register(payload: web::Json<RegisterPayload>, app_state: web::Data<AppState>) -> impl Responder {
        if let Err(_) = payload.validate() { return HttpResponse::UnprocessableEntity();}

        let mut user = User::new(payload.into_inner());
        user.password = match hash(user.password, PASSWORD_STRENGTH) { Ok(hashed) => hashed, Err(_) => return HttpResponse::InternalServerError()};
        
        let conn = &app_state.db_connection;

        match AuthDatabaseController.register_user(user, conn).await {
            Ok(user_id) => {
                let mut response = HttpResponse::Created();
                response.append_header((header::AUTHORIZATION, Jwt.create(user_id)));
                response
            },
            Err(_) => HttpResponse::Conflict()
        }
    }
}