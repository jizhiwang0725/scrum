use crate::models::{Recipe, RecipeListResponse, RecipeSummary};
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use std::sync::Arc;

pub async fn get_recipes(State(state): State<Arc<AppState>>) -> Json<RecipeListResponse> {
    let items = state.recipes.clone();
    let total = items.len();

    Json(RecipeListResponse { items, total })
}

pub async fn get_recipe_by_id(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Recipe>, axum::http::StatusCode> {
    let recipe = state
        .recipes
        .iter()
        .find(|recipe| recipe.id == id)
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    Ok(Json(recipe.clone()))
}

pub async fn search_recipes(
    Path(keyword): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Json<Vec<RecipeSummary>> {
    let results: Vec<RecipeSummary> = state
        .recipes
        .iter()
        .filter(|recipe| recipe.title.contains(&keyword))
        .map(|recipe| RecipeSummary {
            id: recipe.id.clone(),
            title: recipe.title.clone(),
        })
        .collect();

    Json(results)
}
