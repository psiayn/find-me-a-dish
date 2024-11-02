use std::collections::HashSet;

use crate::init::{RankedRecipe, Recipe};


pub fn rank_recipes_by_available_ingredients(
    available_ingredients: Vec<String>,
    recipes: Vec<Recipe>,
) -> Vec<RankedRecipe> {
    let mut ranked_recipes: Vec<RankedRecipe> = Vec::new();

    recipes.iter().for_each(|recipe| {
        let required_ingredients: HashSet<_> = recipe.ingredients.iter().collect();
        let available_ingredients: Vec<String> = available_ingredients
            .clone()
            .into_iter()
            .filter(|ingredient| required_ingredients.contains(ingredient)).collect();
        ranked_recipes.push(RankedRecipe {
            recipe: recipe.clone(),
            rank: available_ingredients.len(),
            available_ingredients: available_ingredients.clone()
        });
    });

    ranked_recipes
}
