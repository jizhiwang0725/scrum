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
    pub title: String,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<Step>,
}

#[derive(Serialize, Clone)]
pub struct RecipeSummary {
    pub id: String,
    pub title: String,
}