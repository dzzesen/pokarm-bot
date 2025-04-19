use crate::dispatcher::{Command, State};
use crate::handlers::{HandlerResult, MyDialogue};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

pub async fn handle_help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    if let Some(user) = msg.from {
        let user_id = user.id;
        bot.send_message(msg.chat.id, format!("user id is {}", user_id))
            .await?;
    } else {
        bot.send_message(msg.chat.id, "user id is not available")
            .await?;
    }
    Ok(())
}

pub async fn handle_add_recipe(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Type a name of new recipe:")
        .await?;
    dialogue.update(State::RecieveName).await?;
    Ok(())
}

pub async fn handle_all_recipes(bot: Bot, msg: Message) -> HandlerResult {
    let keyboard = InlineKeyboardMarkup::new(
        [("Recipe 1", 1), ("Recipe 2", 2), ("Recipe 3", 3)]
            .map(|(recipe, id)| [InlineKeyboardButton::callback(recipe, id.to_string())]),
    );
    bot.send_message(msg.chat.id, "List of your recipes:")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn handle_find_recipe(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Type a search query:")
        .await?;
    dialogue.update(State::RecieveSearchQuery).await?;
    Ok(())
}
