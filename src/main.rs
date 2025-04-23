pub mod db_queries;
pub mod dispatcher;
pub mod handlers;
pub mod models;

use sea_orm::{Database, DatabaseConnection};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env loading failed");

    let bot = Bot::from_env();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db: DatabaseConnection = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    Dispatcher::builder(bot, dispatcher::get_schema())
        .dependencies(dptree::deps![InMemStorage::<dispatcher::State>::new(), db])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
