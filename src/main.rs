use std::env;
use actix::Actor;
use actix_cors::Cors;
use actix_web::{HttpServer, App, middleware::Logger, web::{self, Data}};
use dotenvy::dotenv;
use models::websockets::lobby::Lobby;
use sqlx::postgres::PgPoolOptions;



// Project modules 
// ----------------------------------------------------------------
pub mod routes;
pub mod models;
pub mod handlers;
pub mod utils;
// ----------------------------------------------------------------

#[actix_rt::main]
async fn main() -> Result<(), std::io::Error>{
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    

    let chat_server = Lobby::default().start(); //create and spin up a lobby



    let server_url = env::var("SERVER_URL")
        .expect("SERVER_URL must be set");
    
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");


    let _allowed_origin = env::var("ALLOWED_ORIGIN")
        .expect("ALLOWED_ORIGIN must be set");

    
    let pool = PgPoolOptions::new()
        .max_connections(128)
        .connect(&database_url)
        .await
        .unwrap_or_else(|e|panic!("Can't get a connection with DB. {:?}", e));

    
    HttpServer::new(move || {

        App::new()

            .app_data(Data::new(chat_server.clone()))

            // Set up DB pool to be used with web::Data<Pool> extractor
            .app_data(Data::new(pool.clone()))

            // Maximum of data
            .app_data(web::JsonConfig::default().limit(4096))

            // Logger
            .wrap(Logger::default())

            // CORS 
            .wrap(
                Cors::default()
                    // .allowed_origin(&allowed_origin)
                    .allow_any_origin()
                    .allow_any_header()
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .supports_credentials()
                    .max_age(3600)
            )

            // Routes
            .configure(routes::routes_factory)


    })
    .bind(server_url)?
    .run()
    .await
}
