use jsonwebtoken::{Algorithm, Header, Validation, encode, decode, DecodingKey, EncodingKey};
use std::time::Duration;
use actix_web::HttpRequest;
use chrono::Utc;
use stayalive_api::utils::constants::{JWT_EXPIRATION_DURATION, UNAUTHORIZED};
use actix_web::HttpResponse;
use super::structs::JwtResponse;
use crate::modules::auth::controller::Jwt;


impl JwtResponse{
    pub fn unauthorized() -> JwtResponse {
        JwtResponse {
            user_id: String::from(""),
            error_status: Some(401),
            exp: 0, 
        }
    }
}


impl Jwt{
    pub fn create(&self, user_id: String ) -> String {
        let jwt_secret_key = &self.read_env();
        let key = EncodingKey::from_secret(jwt_secret_key.as_ref());
    
        let expiration = Utc::now() + Duration::from_secs(JWT_EXPIRATION_DURATION);
        let expiration_timestamp = expiration.timestamp();
    
        encode(&Header::new(Algorithm::HS256), &JwtResponse {
            user_id,
            error_status: None,
            exp: expiration_timestamp, 
        }, &key).unwrap()
    }


    pub fn get_user_id_from_token(&self, req: HttpRequest) -> Result<String, bool> {
        match req.headers().get("Authorization").and_then(|auth_header| auth_header.to_str().ok()) {
            Some(auth_value) if auth_value.starts_with("Bearer ") => {
                let token = auth_value.trim_start_matches("Bearer ");
                
                let jwt_response = match self.decode(token){
                    Ok(data) => data,
                    Err(_) => return Err(UNAUTHORIZED)
                };

                let user_id = jwt_response.user_id;
                Ok(user_id)
            }

            Some(_) | None => Err(UNAUTHORIZED)
        }
    }

    
    pub fn check_auth(&self, request: HttpRequest) -> Result<String, HttpResponse>{
        match self.get_user_id_from_token(request) {
            Ok(user_id) => Ok(user_id),
            Err(_) => Err(HttpResponse::Unauthorized().into()),
        }
    }


    fn decode(&self, token: &str) -> Result<JwtResponse, JwtResponse> {
        let jwt_secret_key = &self.read_env();
        let key = DecodingKey::from_secret(jwt_secret_key.as_ref());

        let decoded =  match decode::<JwtResponse>(&token, &key, &Validation::new(Algorithm::HS256)){
            Ok(data) => data,
            Err(_) => return Err(JwtResponse::unauthorized())
        };
        
        Ok(decoded.claims)
    }


    fn read_env(&self) -> String{
        dotenv::var("JWT_SECRET_KEY").unwrap()
    }
}