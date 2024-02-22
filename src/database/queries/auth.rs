use serde::Deserialize;
use surrealdb::sql::Thing;


pub const SELECT_PASSWORD: &str = "SELECT password, id from users where email = '$id0'";

#[derive(Deserialize)]
pub struct SelectPasswordQueryResponse{
    pub id: Thing,
    pub password: String,
}