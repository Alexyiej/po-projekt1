use actix_web::{Responder, http::header, HttpRequest, HttpResponse};
use crate::modules::auth::controller::Jwt;


impl Jwt{
    pub async fn refresh_token(request: HttpRequest) -> impl Responder{
        let user_id = match Jwt.get_user_id_from_token(request){
            Ok(user_id) => user_id,
            Err(_) => return HttpResponse::Unauthorized(),

        };
        
        let mut response = HttpResponse::Ok();
        response.append_header((header::AUTHORIZATION, Jwt.create(user_id)));
        response
    }
}