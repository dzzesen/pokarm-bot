use teloxide::{
    dispatching::{UpdateHandler, dialogue, dialogue::InMemStorage},
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    RecieveName,
    RecieveDescription {
        name: String,
    },
    RecieveRecipe {
        name: String,
        description: String,
    },
    RecieveSearchQuery,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "show all pecipes.")]
    AllRecipes,
    #[command(description = "add a pecipe.")]
    AddRecipe,
    #[command(description = "find a pecipe by ingredient.")]
    FindRecipe,
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(handle_help_cmd))
        .branch(case![Command::AddRecipe].endpoint(handle_add_recipe_cmd))
        .branch(case![Command::AllRecipes].endpoint(handle_all_recipes_cmd))
        .branch(case![Command::FindRecipe].endpoint(handle_find_recipe_cmd));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::RecieveName].endpoint(handle_name_msg))
        .branch(case![State::RecieveDescription { name }].endpoint(handle_description_msg))
        .branch(case![State::RecieveRecipe { name, description }].endpoint(handle_recipe_msg))
        .branch(case![State::RecieveSearchQuery].endpoint(handle_search_query_msg))
        .branch(dptree::endpoint(handle_invalid_msg));

    dialogue::enter::<Update, InMemStorage<State>, State, _>().branch(message_handler)
}

async fn handle_help_cmd(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

async fn handle_add_recipe_cmd(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Type a name of new recipe:")
        .await?;
    dialogue.update(State::RecieveName).await?;
    Ok(())
}

async fn handle_all_recipes_cmd(bot: Bot, msg: Message) -> HandlerResult {
    let keyboard = InlineKeyboardMarkup::new(
        [("Recipe 1", 1), ("Recipe 2", 2), ("Recipe 3", 3)]
            .map(|(recipe, id)| [InlineKeyboardButton::callback(recipe, id.to_string())]),
    );
    bot.send_message(msg.chat.id, "List of your recipes:")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

async fn handle_find_recipe_cmd(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Type a search query:")
        .await?;
    dialogue.update(State::RecieveSearchQuery).await?;
    Ok(())
}

async fn handle_name_msg(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
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

async fn handle_description_msg(
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

async fn handle_recipe_msg(
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

async fn handle_search_query_msg(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
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

async fn handle_invalid_msg(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Unable to handle the message. Type /help to see the usage.",
    )
    .await?;
    Ok(())
}
