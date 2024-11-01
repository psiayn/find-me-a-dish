use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Ingredients {
    name: String,
    unit_of_measure: String,
    category: String,
}

#[derive(Deserialize, Debug)]
pub struct AvailableIngredients {
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Recipes {
    recipe: String,
    instructions: String,
    ingredients: String,
}

fn initialize() {
    let ingredients = csv::Reader::from_path("data/ingredients.csv")
        .unwrap()
        .deserialize()
        .collect::<Result<Vec<Ingredients>, _>>()
        .unwrap();
    let available_ingredients = csv::Reader::from_path("data/available_ingredients.csv")
        .unwrap()
        .deserialize()
        .collect::<Result<Vec<AvailableIngredients>, _>>()
        .unwrap();
    let recipes = csv::Reader::from_path("data/recipes.csv")
        .unwrap()
        .deserialize()
        .collect::<Result<Vec<Recipes>, _>>()
        .unwrap();

    println!("Ingredients: {ingredients:#?}");
    println!("AvailableIngredients: {available_ingredients:#?}");
    println!("Recipes: {recipes:#?}");
}

fn main() {
    initialize();
}
