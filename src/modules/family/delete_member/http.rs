use actix_web::{web, Responder, HttpRequest};
use stayalive_api::database::queries::family::DELETE_FAMILY_MEMBER_QUERY;
use crate::modules::family::controller::FamilyController;
use crate::AppState;


impl FamilyController{
    pub async fn delete_family_member(request: HttpRequest, path_data: web::Path<String> ,app_state: web::Data<AppState>) -> impl Responder{
        Self.update_family_member(request, path_data, app_state, DELETE_FAMILY_MEMBER_QUERY.to_string()).await
    }
}