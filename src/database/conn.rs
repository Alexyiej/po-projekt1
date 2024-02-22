use surrealdb::error::Db;
use surrealdb::Surreal;
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Error;
use surrealdb::sql::Thing;
use surrealdb::Response;
use serde::{Deserialize, Serialize};


pub struct DatabaseConnection;
pub struct QuerySelector;
pub struct DatabaseConfig{
    host: String,
    port: u32,
    username: String,
    password: String,
    database: String,
    namespace: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}


impl DatabaseConnection{
    pub async fn new(&self) -> Result<Surreal<Client>, Error> {
        let config = self.read_config();
        
        let url = format!("{}:{}", config.host, config.port);
        let connection = Surreal::new::<Ws>(url).await?;
    
        connection.signin(Root {
            username: &config.username,
            password: &config.password,
        }).await?;
    
        connection
          .use_ns(&config.namespace)
          .use_db(&config.database).await?;
    
        Ok(connection)
    }


    fn read_config(&self) -> DatabaseConfig{
        DatabaseConfig{
            host: dotenv::var("DB_HOST").unwrap(),
            port: dotenv::var("DB_PORT").unwrap().parse().unwrap(),
            database: dotenv::var("DB_DATABASE").unwrap(),
            username: dotenv::var("DB_USERNAME").unwrap(),
            password: dotenv::var("DB_PASSWORD").unwrap(),
            namespace: dotenv::var("NAMESPACE").unwrap(),
        }
    }
}


impl QuerySelector{
    pub async fn query<T>(&self, conn: &Surreal<Client>, query: String) -> Result<T, Error> where T: serde::de::DeserializeOwned{
        let mut result: Response = match conn.query(query).await{
            Ok(data) => data,
            Err(e) => {println!("{e}"); return Err(e)},
        };

        let data: T = match result.take(0){
            Ok(data) => if let Some(data) = data { data } else { return Err(Error::Db(Db::DbEmpty)) },
            Err(e) => {println!("{e}"); return Err(e)},
        };

        Ok(data)
    }

    
    pub fn create_query(&self, mut query: String, ids: Vec<String>) -> String{
        for (i, id) in ids.iter().enumerate() {
            let new_query = query.replace(&format!("$id{i}"), id);
            query = new_query;
        }
        query
    }


    pub fn create_id(&self, query_result: Thing) -> String {
        format!("{}:{}", query_result.tb, query_result.id)
    }
}