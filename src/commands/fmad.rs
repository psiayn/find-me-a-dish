use std::collections::HashSet;

use serenity::all::{Colour, CreateEmbedFooter};
use serenity::builder::{CreateCommand, CreateEmbed};
use serenity::model::application::ResolvedOption;

use crate::init::initialize;
use crate::rank::rank_recipes_by_available_ingredients;

pub fn run() -> Vec<CreateEmbed> {
    let (available_ingredients, recipes) = initialize();

    let mut ranked_recipes = rank_recipes_by_available_ingredients(available_ingredients, recipes);
    ranked_recipes.sort_by_key(|ranked_recipe| ranked_recipe.rank);
    ranked_recipes.reverse();

    let mut output: String = Default::default();
    ranked_recipes.iter().for_each(|recipe| {
        output.push_str(recipe.to_string().as_str());
        output.push_str("\n\n");
    });

    let mut embeds: Vec<CreateEmbed> = Vec::new();

    for (i, ranked_recipe) in ranked_recipes.iter().enumerate() {
        let no_of_available_ingredients = ranked_recipe.rank.to_string()
            + "/"
            + ranked_recipe.recipe.ingredients.len().to_string().as_str();

        let available_ingredients: HashSet<String> = HashSet::from_iter(ranked_recipe.available_ingredients.clone().into_iter());
        let required_ingredients: HashSet<String> = HashSet::from_iter(ranked_recipe.recipe.ingredients.clone().into_iter());

        let mut missing_ingredients: Vec<String> = required_ingredients.difference(&available_ingredients).map(String::to_string).collect();


        let mut ingredients_field: Vec<String> = available_ingredients.into_iter().map(|ingredient| {
            format!("**{ingredient}**")
        }).collect();

        ingredients_field.append(&mut missing_ingredients);


        let footer = format!("Page {}/{}\n Click on 👈 👉 to navigate\n", i+1, ranked_recipes.len());
        let embed = CreateEmbed::new()
            .title("Recipe Card")
            .description(ranked_recipe.recipe.recipe.clone())
            .colour(Colour::from_rgb(245, 239, 66))
            .field(
                "Number of Available Ingredients",
                no_of_available_ingredients,
                false,
            )
            .field(
                "Recipe Ingredients",
                ingredients_field.join(", "),
                false,
            )
            .field(
                "Recipe Instruction",
                ranked_recipe.recipe.instructions.clone(),
                false,
            )
            .footer(CreateEmbedFooter::new(footer))
            .timestamp(chrono::Utc::now());
        embeds.push(embed);
    }

    embeds
}

pub fn register() -> CreateCommand {
    CreateCommand::new("fmad").description("Find me a dish pwetty pweeeezz 🥺")
}
