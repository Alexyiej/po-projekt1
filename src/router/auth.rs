use actix_web::web::{post, ServiceConfig};
use crate::Router;
use crate::modules::auth::controller::{Auth, Jwt};


impl Router{
    pub fn auth(router_cfg: &mut ServiceConfig) {
        router_cfg
        .route("/register", post().to(Auth::register))
        .route("/login", post().to(Auth::login))
        .route("/logout", post().to(Auth::logout))
        .route("/refresh-token", post().to(Jwt::refresh_token));
    }
}