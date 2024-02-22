use stayalive_api::database::{conn::QuerySelector, queries::profile::SelectProfileQueryResult};
use serde::Serialize;


#[derive(Serialize)]
pub struct FamilyMember{
    pub id: String,
    pub created_at: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub email: String,
    pub family_id: String,
    pub status: bool,
}


impl FamilyMember{
    pub fn from(profile_query_result: SelectProfileQueryResult) -> Self {
        let SelectProfileQueryResult {id, created_at, first_name, last_name, role, email, family_id, status} = profile_query_result;

        Self{
            id: QuerySelector.create_id(id),
            created_at,
            first_name,
            last_name,
            role,
            email,
            family_id,
            status
        }
    }
} 