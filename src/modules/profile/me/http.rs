use crate::modules::{family::get_members::structs::FamilyMember, profile::controller::ProfileController};
use crate::AppState;
use crate::modules::auth::controller::Jwt;
use crate::modules::profile::db::ProfileDatabaseController;
use actix_web::{web, Responder, HttpRequest, HttpResponse};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;


impl ProfileController{
    pub async fn me(request: HttpRequest, app_state: web::Data<AppState>) -> impl Responder{
        let user_id = match Jwt.check_auth(request){
            Ok(data) => data,
            Err(response) => return response,
        };

        let conn = &app_state.db_connection;

        let profile = match ProfileController.profile_by_user_id(conn, user_id).await{
            Ok(data) => data,
            Err(_) => return HttpResponse::InternalServerError().into(),
        };

        return HttpResponse::Ok().json(profile);
    }


    async fn profile_by_user_id(&self, conn: &Surreal<Client>, user_id: String) -> Result<FamilyMember, ()>{
        let profile = match ProfileDatabaseController.select_user_by_user_id(conn, user_id).await{
            Ok(data) => data,
            Err(_) => return Err(()),
        };
        
        Ok(FamilyMember::from(profile))
    }
}