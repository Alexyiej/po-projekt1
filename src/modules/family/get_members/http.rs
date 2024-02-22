use actix_web::{web, Responder, HttpRequest, HttpResponse};
use crate::modules::family::controller::FamilyController;
use crate::modules::family::db::FamilyDatabaseController;
use crate::modules::auth::controller::Jwt;
use crate::AppState;
use super::structs::FamilyMember;


impl FamilyController{
    pub async fn get_family(request: HttpRequest, app_state: web::Data<AppState>) -> impl Responder{
        let user_id = match Jwt.check_auth(request){
            Ok(data) => data,
            Err(response) => return response,
        };

        let database = FamilyDatabaseController{
            conn: app_state.db_connection.clone()
        };

        let family_id = match database.select_family_id(user_id).await{
            Ok(data) => data,
            Err(_) => return HttpResponse::InternalServerError().into(),
        };
        
        if family_id == String::from(""){ return HttpResponse::NotFound().into(); } 

        let family_profiles = match database.select_family(family_id).await {
            Ok(data) => data,
            Err(_) => return HttpResponse::InternalServerError().into(),
        };
        
        let family: Vec<FamilyMember> = family_profiles.into_iter().map(|profile| FamilyMember::from(profile)).collect();
        
        return HttpResponse::Ok().json(family);
    }
}


