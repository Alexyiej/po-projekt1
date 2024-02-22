use actix_web::web::{get, delete, put, ServiceConfig};
use crate::Router;
use crate::modules::family::controller::FamilyController;


impl Router{
    pub fn family(router_cfg: &mut ServiceConfig) {
        router_cfg
        .route("", get().to(FamilyController::get_family))
        .route("/{user_id}", delete().to(FamilyController::delete_family_member))
        .route("/{user_id}", put().to(FamilyController::add_family_member));
    }
}