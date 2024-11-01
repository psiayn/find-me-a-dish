use serde::{Deserialize, Deserializer};
use std::{collections::HashSet, fs};

fn vec_string_from_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let keywords = String::deserialize(deserializer)?;
    match keywords.len() {
        0 => Ok(vec![]),
        _ => Ok(keywords.split(",").map(|s| s.trim().to_string()).collect()),
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Recipe {
    recipe: String,
    instructions: String,
    #[serde(deserialize_with = "vec_string_from_string")]
    ingredients: Vec<String>,
}

#[derive(Debug)]
pub struct RankedRecipe {
    recipe: Recipe,
    rank: usize,
}

fn initialize() -> (Vec<String>, Vec<Recipe>) {
    let available_ingredients = csv::Reader::from_path("data/available_ingredients.csv")
        .unwrap()
        .deserialize()
        .collect::<Result<Vec<String>, _>>()
        .unwrap();
    let recipes = csv::Reader::from_path("data/recipes.csv")
        .unwrap()
        .deserialize()
        .collect::<Result<Vec<Recipe>, _>>()
        .unwrap();

    println!("AvailableIngredients: {available_ingredients:#?}");
    println!("Recipes: {recipes:#?}");

    (available_ingredients, recipes)
}

fn rank_recipes_by_available_ingredients(
    available_ingredients: Vec<String>,
    recipes: Vec<Recipe>,
) -> Vec<RankedRecipe> {
    let mut ranked_recipes: Vec<RankedRecipe> = Vec::new();

    recipes.iter().for_each(|recipe| {
        let required_ingredients: HashSet<_> = recipe.ingredients.iter().collect();
        let available_count = available_ingredients
            .iter()
            .filter(|ingredient| required_ingredients.contains(ingredient))
            .count();
        ranked_recipes.push(RankedRecipe {
            recipe: recipe.clone(),
            rank: available_count,
        });
    });

    ranked_recipes
}

fn main() {
    let (available_ingredients, recipes) = initialize();

    let mut ranked_recipes = rank_recipes_by_available_ingredients(available_ingredients, recipes);
    ranked_recipes.sort_by_key(|ranked_recipe| ranked_recipe.rank);
    ranked_recipes.reverse();
    
    println!("{ranked_recipes:#?}");
    
}
