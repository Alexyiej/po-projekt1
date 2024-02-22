use actix_web::web::{get, ServiceConfig};
use crate::Router;
use crate::modules::profile::controller::ProfileController;

impl Router{
    pub fn profile(router_cfg: &mut ServiceConfig) {
        router_cfg
        .route("/me", get().to(ProfileController::me));
    }
}