use uuid::Uuid;
use super::structs::{RegisterPayload, User};


impl User{
    pub fn new(payload: RegisterPayload) -> Self{
        let RegisterPayload {password, email, first_name, family_id, last_name, role} = payload;
        
        Self{
            password,
            email,
            created_at: chrono::Utc::now(),
            first_name,
            last_name,
            role,
            family_id: if let Some(family_id) = family_id { family_id } else { String::from(Uuid::new_v4()) },
            status: true
        }
    }
}