use axum::{extract::State, Json};
use libsql::Database;
use std::sync::Arc;

use crate::models::Category;

pub async fn list(State(db): State<Arc<Database>>) -> Json<Vec<Category>> {
    let conn = db.connect().unwrap();
    let mut rows = conn
        .query("SELECT id, name, slug, sort_order FROM categories ORDER BY sort_order", ())
        .await
        .unwrap();

    let mut list = Vec::new();
    while let Ok(Some(row)) = rows.next().await {
        if let Ok(cat) = Category::from_row(&row) {
            list.push(cat);
        }
    }
    Json(list)
}
