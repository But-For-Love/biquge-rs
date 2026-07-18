mod db;
mod handlers;
mod models;
mod seed;

use axum::{routing::get, Router};
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let db_path = std::env::var("BIQUGE_DB")
        .unwrap_or_else(|_| String::from("data/biquge.db"));

    // Ensure data directory exists
    if let Some(parent) = PathBuf::from(&db_path).parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let database = Arc::new(db::connect(&db_path).await);
    db::init_db(&database).await;
    seed::seed(&database).await;

    println!("Backend running on http://localhost:3000");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/home", get(handlers::home::home_data))
        .route("/api/categories", get(handlers::categories::list))
        .route("/api/novels", get(handlers::novels::list))
        .route("/api/novels/:id", get(handlers::novels::detail))
        .route("/api/chapters/:novel_id/:chapter_id", get(handlers::chapters::read))
        .route("/api/search", get(handlers::search::search))
        .route("/api/rankings", get(handlers::rankings::list))
        .layer(cors)
        .with_state(database);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
