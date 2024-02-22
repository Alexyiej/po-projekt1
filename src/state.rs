use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use crate::database::conn::DatabaseConnection;


pub struct AppState {
    pub db_connection: Surreal<Client>,
}


impl AppState {
    pub async fn new() -> Self {
        let db_connection = match DatabaseConnection::new(&DatabaseConnection).await{
            Ok(conn) => conn,
            Err(e) => panic!("{}", e.to_string()),
        };

        AppState {
            db_connection,
        }
    }
}