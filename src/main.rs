use actix_web::{App, HttpServer};
use diesel::{pg::PgConnection, Connection};
use dotenvy::dotenv;
use utils::get_env;

mod models;
mod schemas;
mod utils;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let connection_string = get_env("DATABASE_URL");
    let _g = PgConnection::establish(&connection_string);

    HttpServer::new(move || App::new())
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
