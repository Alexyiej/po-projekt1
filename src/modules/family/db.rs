use stayalive_api::database::queries::family::{SelectFamilyIdQueryResult, SELECT_FAMILY_ID_QUERY, SELECT_FAMILY_QUERY};
use stayalive_api::database::queries::profile::SelectProfileQueryResult;
use stayalive_api::database::conn::QuerySelector;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;


pub struct FamilyDatabaseController{
    pub conn: Surreal<Client>,
}


impl FamilyDatabaseController{
    pub async fn select_family_id(&self, user_id: String) -> Result<String, ()>{
        let query = QuerySelector.create_query(SELECT_FAMILY_ID_QUERY.to_string(), vec![user_id]);
        
        let SelectFamilyIdQueryResult { family_id } = match QuerySelector.query(&self.conn, query).await {
            Ok(data) => data,
            Err(_) => return Err(()),
        };

        Ok(family_id)
    }


    pub async fn select_family(&self, family_id: String) -> Result<Vec<SelectProfileQueryResult>, ()>{
        let query = QuerySelector.create_query(SELECT_FAMILY_QUERY.to_string(), vec![family_id]);

        let mut result = match self.conn.query(query).await{
            Ok(data) => data,
            Err(_) => return Err(()),
        };
        
        let family_data: Vec<SelectProfileQueryResult> = match result.take(0){
            Ok(data) => data, 
            Err(_) => return Err(()),
        };

        Ok(family_data)
    }


    pub async fn update_member(&self, family_id: String, user_id: String, query: String) -> Result<(),()> {
        let new_query = QuerySelector.create_query(query, vec![user_id, family_id]);

        let result: SelectProfileQueryResult = match QuerySelector.query(&self.conn, new_query).await {
            Ok(data) => data,
            Err(_) => return Err(()),
        };

        drop(result);
        Ok(())
    }
}