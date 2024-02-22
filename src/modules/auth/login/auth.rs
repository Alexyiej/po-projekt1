use bcrypt::verify;
use surrealdb::{engine::remote::ws::Client, Surreal};
use stayalive_api::database::queries::auth::SelectPasswordQueryResponse;
use crate::modules::auth::{controller::AuthController, db::AuthDatabaseController};


impl AuthController{
    pub async fn check_login(&self, email: String, password: String, conn: &Surreal<Client>) -> Result<(bool, String), u16> {
        let query_response: SelectPasswordQueryResponse = match AuthDatabaseController.get_user_password(conn, email).await {
            Ok(data) => data,
            Err(e) => return Err(e),
        };

        let hashed_password = query_response.password;
        
        let table = query_response.id.tb;
        let id = query_response.id.id;
        let user_id = format!("{}:{}", table, id);

        Ok((self.compare_passwords(hashed_password, password), user_id))
    }
    
    
    fn compare_passwords(&self, hashed_password: String, password: String) -> bool {
        match verify(password, &hashed_password) {
            Ok(valid) => valid,
            _=> false,
        }
    }
}