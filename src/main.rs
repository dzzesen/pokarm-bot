mod dispatcher;
mod handlers;

use teloxide::{prelude::*, dispatching::dialogue::InMemStorage};

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    Dispatcher::builder(bot, dispatcher::get_schema())
        .dependencies(dptree::deps![InMemStorage::<dispatcher::State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
