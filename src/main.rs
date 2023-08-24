use dotenvy::dotenv;
use sea_orm::Database;
use utils::get_env;

mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let user = get_env("PG_USER");
    let password = get_env("PG_PASSWORD");
    let host = get_env("PG_HOST");

    let connection_string = format!("postgresql://{}:{}@{}", user, password, host);

    let db = Database::connect(connection_string).await;
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
