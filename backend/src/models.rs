use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Ingredient {
    pub name: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Step {
    pub number: u32,
    pub text: String,
    pub image_urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub id: String,
    pub url: String,
    pub title: String,
    pub author: String,
    pub author_url: String,
    pub description: String,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<Step>,
    pub tips: String,
    pub cover_image_urls: Vec<String>,
}

#[derive(Serialize, Clone)]
pub struct RecipeListResponse {
    pub items: Vec<Recipe>,
    pub total: usize,
}

#[derive(Serialize, Clone)]
pub struct RecipeSummary {
    pub id: String,
    pub title: String,
}
