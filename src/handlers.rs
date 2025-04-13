use crate::dispatcher::{Command, State};
use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn handle_help_cmd(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

pub async fn handle_add_recipe_cmd(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Type a name of new recipe:")
        .await?;
    dialogue.update(State::RecieveName).await?;
    Ok(())
}

pub async fn handle_all_recipes_cmd(bot: Bot, msg: Message) -> HandlerResult {
    let keyboard = InlineKeyboardMarkup::new(
        [("Recipe 1", 1), ("Recipe 2", 2), ("Recipe 3", 3)]
            .map(|(recipe, id)| [InlineKeyboardButton::callback(recipe, id.to_string())]),
    );
    bot.send_message(msg.chat.id, "List of your recipes:")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn handle_find_recipe_cmd(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Type a search query:")
        .await?;
    dialogue.update(State::RecieveSearchQuery).await?;
    Ok(())
}

pub async fn handle_name_msg(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(name) => {
            bot.send_message(
                msg.chat.id,
                format!("Ok, the name of a new recipe is\n{name}\n\nType a description:"),
            )
            .await?;
            dialogue.update(State::RecieveDescription { name }).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please send me a name for a new recipe.")
                .await?;
        }
    }
    Ok(())
}

pub async fn handle_description_msg(
    bot: Bot,
    dialogue: MyDialogue,
    name: String,
    msg: Message,
) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(description) => {
            bot.send_message(
                msg.chat.id,
                format!("Ok, the name of a new recipe is\n{name},\n\nAnd the description of a new recipe is\n{description}\n\nType a recipe:"),
            )
            .await?;
            dialogue
                .update(State::RecieveRecipe { name, description })
                .await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Please send me a description for a new recipe.",
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn handle_recipe_msg(
    bot: Bot,
    dialogue: MyDialogue,
    (name, description): (String, String),
    msg: Message,
) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(recipe) => {
            bot.send_message(
                msg.chat.id,
                format!("Ok, the name of a new recipe is\n{name},\n\nAnd the description of a new recipe is\n{description}\n\nThe new recipe is\n{recipe}\n\nUnfortunatelly, this feature is on development =)"),
            )
            .await?;
            dialogue.update(State::default()).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please send me a new recipe.")
                .await?;
        }
    }
    Ok(())
}

pub async fn handle_search_query_msg(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
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

pub async fn handle_invalid_msg(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Unable to handle the message. Type /help to see the usage.",
    )
    .await?;
    Ok(())
}
