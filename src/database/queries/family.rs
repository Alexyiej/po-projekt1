use serde::{Deserialize, Serialize};


pub const SELECT_FAMILY_ID_QUERY: &str = "SELECT family_id from users where id = $id0;";

#[derive(Deserialize, Serialize, Debug)]
pub struct SelectFamilyIdQueryResult{
    pub family_id: String,
}


pub const SELECT_FAMILY_QUERY: &str = "SELECT id, created_at, first_name, last_name, role, email, family_id, status from users where family_id = '$id0';";
pub const DELETE_FAMILY_MEMBER_QUERY: &str = "UPDATE users SET family_id = '' where id = $id0 and family_id = '$id1';";
pub const ADD_USER_TO_FAMILY_QUERY: &str = "UPDATE users SET family_id = '$id1' where id = $id0;";