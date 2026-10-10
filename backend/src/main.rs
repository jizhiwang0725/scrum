use axum::{
    extract::{Path, State},
    routing::get,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone)]
struct Ingredient {
    name: String,
    amount: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Step {
    number: u32,
    text: String,
    image_urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Recipe {
    id: String,
    title: String,
    ingredients: Vec<Ingredient>,
    steps: Vec<Step>,
}

#[derive(Serialize, Clone)]
struct RecipeSummary {
    id: String,
    title: String,
}

struct AppState {
    recipes: Vec<Recipe>,
}

fn load_recipes() -> Result<Vec<Recipe>, String> {
    let json: String = fs::read_to_string("recipes.json")
        .map_err(|error| error.to_string())?;

    let recipes: Vec<Recipe> = serde_json::from_str(&json)
        .map_err(|error| error.to_string())?;

    Ok(recipes)
}

async fn get_recipes(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<Recipe>> {
    Json(state.recipes.clone())
}

async fn get_recipe_by_id(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Recipe>, axum::http::StatusCode> {
    let recipe: &Recipe = state.recipes
        .iter()
        .find(|recipe: &&Recipe| recipe.id == id)
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    Ok(Json(recipe.clone()))
}

async fn search_recipes(
    Path(keyword): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Json<Vec<RecipeSummary>> {
    let results: Vec<RecipeSummary> = state.recipes
        .iter()
        .filter(|recipe: &&Recipe| recipe.title.contains(&keyword))
        .map(|recipe: &Recipe| RecipeSummary {
            id: recipe.id.clone(),
            title: recipe.title.clone(),
        })
        .collect();

    Json(results)
}

#[derive(Serialize)]
struct HealthResponse{
    status: &'static str,
}

async fn health() -> Json<HealthResponse>{
    Json(HealthResponse { status: "ok"}) //Return Json Response
}

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let recipes = load_recipes()
    .expect("Failed to load recipes");
    
    let state = Arc::new(AppState { recipes }); 

    let app = Router::new()
    .route("/api/health", get(health))
    .route("/api/recipes", get(get_recipes))
    .route("/api/recipes/{id}", get(get_recipe_by_id))
    .route("/api/search/{keyword}", get(search_recipes))
    .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        //listen to 3000 port

    println!("Server: http://127.0.0.1:3000");
    axum::serve(listener, app).await //Receive Request
}
