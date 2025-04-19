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
        .branch(case![Command::Help].endpoint(commands::handle_help))
        .branch(case![Command::AddRecipe].endpoint(commands::handle_add_recipe))
        .branch(case![Command::AllRecipes].endpoint(commands::handle_all_recipes))
        .branch(case![Command::FindRecipe].endpoint(commands::handle_find_recipe));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::RecieveName].endpoint(create_recipe::handle_name))
        .branch(
            case![State::RecieveDescription { name }]
                .endpoint(create_recipe::handle_description),
        )
        .branch(
            case![State::RecieveRecipe { name, description }]
                .endpoint(create_recipe::handle_recipe),
        )
        .branch(case![State::RecieveSearchQuery].endpoint(handle_search_query))
        .branch(dptree::endpoint(handle_invalid));

    dialogue::enter::<Update, InMemStorage<State>, State, _>().branch(message_handler)
}
