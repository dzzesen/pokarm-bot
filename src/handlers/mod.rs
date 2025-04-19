pub mod commands;
pub mod create_recipe;

use crate::dispatcher::State;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn handle_search_query(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(query) => {
            bot.send_message(msg.chat.id, format!("Ok, your search query is {query}! Unfortunatelly, this feature is on development =)")).await?;
            dialogue.update(State::default()).await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Please send me a search query to find some recipe.",
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn handle_invalid(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Unable to handle the message. Type /help to see the usage.",
    )
    .await?;
    Ok(())
}
