use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;


pub const SELECT_ME: &str = "SELECT id, created_at, first_name, last_name, role, email, family_id, status from users where id = $id0;";

#[derive(Deserialize, Serialize, Debug)]
pub struct SelectProfileQueryResult{
    pub id: Thing,
    pub created_at: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub email: String,
    pub family_id: String,
    pub status: bool,
}