use surrealdb::Error as SurrealError;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use stayalive_api::database::conn::Record;
use stayalive_api::database::queries::auth::{SelectPasswordQueryResponse, SELECT_PASSWORD};
use stayalive_api::database::conn::QuerySelector;
use crate::modules::auth::register::structs::User;


pub struct AuthDatabaseController;

impl AuthDatabaseController{
    pub async fn register_user(&self, user: User, conn: &Surreal<Client>) -> Result<String, SurrealError>{
        match conn.create("users").content(user).await {
            Ok(db_result) => {
                let created_user: &Record = &db_result[0];
                let created_info: &Thing = &created_user.id;
                
                Ok(QuerySelector.create_id(created_info.to_owned()))
            },

            Err(e) => return Err(e),
        }
    }


    pub async fn get_user_password(&self, conn: &Surreal<Client>, email: String) -> Result<SelectPasswordQueryResponse, u16>{
        let query = QuerySelector.create_query(SELECT_PASSWORD.to_string(), vec![email]);

        let query_response: SelectPasswordQueryResponse = match QuerySelector.query(conn, query).await{
            Ok(data) => data,
            Err(e) => {println!("{:?}", e); return Err(500) },
        };

        Ok(query_response)
    }
}
