use serde::{Deserialize, Serialize};


#[derive(Deserialize,Debug, Serialize)]
pub struct JwtResponse {
    pub user_id: String,
    pub error_status: Option<u16>,
    pub exp: i64, 
}
