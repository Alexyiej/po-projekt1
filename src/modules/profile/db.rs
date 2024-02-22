use stayalive_api::database::queries::profile::{SELECT_ME, SelectProfileQueryResult};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use stayalive_api::database::conn::QuerySelector;


pub struct ProfileDatabaseController;

impl ProfileDatabaseController{ 
    pub async fn select_user_by_user_id(&self, conn: &Surreal<Client>, user_id: String) -> Result<SelectProfileQueryResult, ()>{
        let query = QuerySelector.create_query(SELECT_ME.to_string(), vec![user_id]);

        let user: SelectProfileQueryResult = match QuerySelector.query(conn, query).await{
            Ok(data) => data,
            Err(_) => return Err(()),
        };

        Ok(user)
    }
}