use crate::models::Recipe;

pub fn load_recipes() -> Result<Vec<Recipe>, String> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/recipes.json");

    let json = std::fs::read_to_string(path)
        .map_err(|error| error.to_string())?;

    let recipes: Vec<Recipe> =
        serde_json::from_str(&json).map_err(|error| error.to_string())?;

    Ok(recipes)
}
