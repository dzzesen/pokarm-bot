use crate::handlers::{HandlerResult, MyDialogue};
use crate::{DbPool, dispatcher::State, models::*, schema::*};
use diesel::{RunQueryDsl, SelectableHelper};
use teloxide::prelude::*;

pub async fn handle_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
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

pub async fn handle_description(
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

pub async fn handle_recipe(
    bot: Bot,
    dialogue: MyDialogue,
    (name, description): (String, String),
    msg: Message,
    db_pool: DbPool,
) -> HandlerResult {
    let mut conn = db_pool.get().expect("Failed to get DB connection.");

    match msg.text().map(ToOwned::to_owned) {
        Some(recipe) => {
            bot.send_message(
                msg.chat.id,
                format!("Ok, the name of a new recipe is\n{name},\n\nAnd the description of a new recipe is\n{description}\n\nThe new recipe is\n{recipe}"),
            )
            .await?;

            dialogue.update(State::default()).await?;

            let new_recipe = NewRecipe {
                name: &name,
                description: &description,
                text: &recipe,
            };
            diesel::insert_into(recipes::table)
                .values(&new_recipe)
                .returning(Recipe::as_returning())
                .get_result(&mut conn)
                .expect("Error saving new recipe.");
        }
        None => {
            bot.send_message(msg.chat.id, "Please send me a new recipe.")
                .await?;
        }
    }
    Ok(())
}
