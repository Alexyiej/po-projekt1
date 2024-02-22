use actix_web::{App, web, HttpServer};
use actix_web::web::{get, scope};
use actix_cors::Cors;
use stayalive_api::state::AppState;


pub mod modules{
    pub mod auth{
        pub mod register{
            pub mod http;
            pub mod user;
            pub mod structs;
        }
        pub mod login{
            pub mod http;
            pub mod auth;
            pub mod structs;
        }
        pub mod logout{
            pub mod http;
            pub mod structs;
        }
        pub mod jwt{
            pub mod structs;
            pub mod refresh;
            pub mod token;
        }
        pub mod db;
        pub mod controller;
    }

    pub mod profile{
        pub mod me{
            pub mod http;
        }
        pub mod db;
        pub mod controller;
    }

    pub mod family{
        pub mod add_member{
            pub mod http;
        }
        pub mod delete_member{
            pub mod http;
        }
        pub mod get_members{
            pub mod structs;
            pub mod http;
        }
        pub mod db;
        pub mod controller;
    }
}


mod scripts{
    mod health{
        mod http;
    }
}


mod router{
    pub mod auth;
    pub mod family;
    pub mod profile;
}


pub struct Health;
pub struct Router;


const HOST: &str = "0.0.0.0";
const PORT: u16 = 8081;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState::new().await);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
        )   
        .route("/health", get().to(Health::health))

        .service(scope("/auth").configure(Router::auth)) 
        .service(scope("/profile").configure(Router::profile))
        .service(scope("/family").configure(Router::family))
    })
    .bind((HOST, PORT))?
    .run().await?;

    Ok(())
}