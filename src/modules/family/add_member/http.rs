use actix_web::{web, Responder, HttpRequest};
use stayalive_api::database::queries::family::ADD_USER_TO_FAMILY_QUERY;
use crate::modules::family::controller::FamilyController;
use crate::AppState;


impl FamilyController{
    pub async fn add_family_member(request: HttpRequest, app_state: web::Data<AppState>, path_data: web::Path<String>) -> impl Responder{
        Self.update_family_member(request, path_data, app_state, ADD_USER_TO_FAMILY_QUERY.to_string()).await
    }
}