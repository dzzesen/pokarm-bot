pub mod dispatcher;
pub mod handlers;
pub mod models;

use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env loading failed");

    let bot = Bot::from_env();

    Dispatcher::builder(bot, dispatcher::get_schema())
        .dependencies(dptree::deps![InMemStorage::<dispatcher::State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
