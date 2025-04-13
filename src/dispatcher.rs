use crate::handlers::*;
use teloxide::{
    dispatching::{UpdateHandler, dialogue, dialogue::InMemStorage},
    prelude::*,
    utils::command::BotCommands,
};

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
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "show all pecipes.")]
    AllRecipes,
    #[command(description = "add a pecipe.")]
    AddRecipe,
    #[command(description = "find a pecipe by ingredient.")]
    FindRecipe,
}

pub fn get_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
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
