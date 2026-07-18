use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

mod utils;
mod routes;

#[derive(Debug)]
struct MainError {
    message: String
}



#[actix_web::main] // or #[tokio::main]
async fn main() -> Result<(),MainError> {

    if std::env::var_os("RUST_LOG").is_none() {
        
        unsafe {std::env::set_var("RUST_LOG", "actix_web=info");}
    };

    dotenv::dotenv().ok();
    env_logger::init();

    let address = utils::constants::ADDRESS.clone();
    let port = utils::constants::PORT.clone();

    let db: DatabaseConnection = Database::connect(utils::constants::DATABASE_URL.clone())
    .await
    .map_err(|err| MainError {message: err.to_string()})?;

    Migrator::up(&db, None)
    .await
    .map_err(|err| MainError {message: err.to_string()})?;

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(utils::app_state::AppState{db: db.clone()}))
        .wrap(Logger::default())
        .configure(routes::home_routes::config)
        .configure(routes::auth_routes::config)
        .configure(routes::user_routes::config)
    })
    .bind((address, port))
    .map_err(|err| MainError {message: err.to_string()})?
    .run()
    .await
    .map_err(|err| MainError {message: err.to_string()})
}