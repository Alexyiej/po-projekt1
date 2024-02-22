use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Validate, Deserialize)]
pub struct RegisterPayload{
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(email)]
    pub email: String,

    pub family_id: Option<String>,
    pub first_name: String,
    pub last_name: String, 
    pub role: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct User{
    pub password: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub first_name: String,
    pub last_name: String, 
    pub family_id: String,
    pub role: String,
    pub status: bool
}