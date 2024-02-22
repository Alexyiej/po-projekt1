use crate::AppState;
use crate::modules::auth::controller::Jwt;
use crate::modules::family::db::FamilyDatabaseController;
use actix_web::{web, Responder, HttpRequest, HttpResponse};


pub struct FamilyController;

impl FamilyController{
    pub async fn update_family_member(&self, request: HttpRequest, path_data: web::Path<String> ,app_state: web::Data<AppState>, query: String) -> impl Responder{
        let user_id = match Jwt.check_auth(request){
            Ok(data) => data,
            Err(response) => return response,
        };
        
        let user_id_to_update = path_data.into_inner();

        let database = FamilyDatabaseController{
            conn: app_state.db_connection.clone()
        };

        let family_id = match database.select_family_id(user_id).await{
            Ok(data) => data,
            Err(_) => return HttpResponse::ServiceUnavailable().into(), 
        };

        if family_id == String::from(""){ return HttpResponse::NotFound().into(); }

        match database.update_member(family_id, user_id_to_update, query).await{
            Ok(_) => {},
            Err(_) => return HttpResponse::NotFound().into(), 
        }

        HttpResponse::NoContent().finish()
    }
}

