pub mod dispatcher;
pub mod handlers;
pub mod models;
pub mod schema;

use diesel::{pg::PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;
use std::env;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env loading failed");

    let bot = Bot::from_env();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_manager = ConnectionManager::<PgConnection>::new(db_url);
    let db_pool = r2d2::Pool::builder().max_size(5).build(db_manager).expect("Failed to create pool.");

    Dispatcher::builder(bot, dispatcher::get_schema())
        .dependencies(dptree::deps![InMemStorage::<dispatcher::State>::new(), db_pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
