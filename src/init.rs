use serde::{Deserialize, Deserializer};

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
    pub recipe: String,
    pub instructions: String,
    #[serde(deserialize_with = "vec_string_from_string")]
    pub ingredients: Vec<String>,
}

impl std::fmt::Display for Recipe {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display = "Recipe: ".to_owned() + &self.recipe;
        display.push_str("\nInstructions: \n");
        display.push_str(self.instructions.as_str());
        display.push_str("\n ingredients: \n");
        display.push_str(self.ingredients.join(", ").as_str());
        fmt.write_str(display.as_str()).unwrap();
        Ok(())
    }
}

#[derive(Debug)]
pub struct RankedRecipe {
    pub recipe: Recipe,
    pub rank: usize,
    pub available_ingredients: Vec<String>
}

impl std::fmt::Display for RankedRecipe {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("Rank: ").unwrap();
        fmt.write_str(self.rank.to_string().as_str()).unwrap();
        fmt.write_str("\n\n").unwrap();
        fmt.write_str(self.recipe.to_string().as_str()).unwrap();
        Ok(())
    }
}

pub fn initialize() -> (Vec<String>, Vec<Recipe>) {
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


    (available_ingredients, recipes)
}
