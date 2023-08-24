use dotenvy::dotenv;
use sea_orm::Database;
use utils::get_env;

mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    get_env("DEV_MODE");

    let db = Database::connect("postgresql://postgres:123456@localhost:5432").await;
    match db {
        Ok(_) => {
            println!("Connected")
        }
        Err(err) => {
            // ...
            println!("{}", err.to_string())
        }
    }
}
