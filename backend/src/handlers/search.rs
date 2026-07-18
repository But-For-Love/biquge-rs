use axum::{extract::{Query, State}, Json};
use serde::Deserialize;
use libsql::Database;
use std::sync::Arc;

use crate::models::Novel;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

pub async fn search(
    State(db): State<Arc<Database>>,
    Query(params): Query<SearchQuery>,
) -> Json<Vec<Novel>> {
    let query = params.q.unwrap_or_default();
    if query.is_empty() {
        return Json(vec![]);
    }
    let pattern = format!("%{}%", query);

    let conn = db.connect().unwrap();
    let mut rows = conn
        .query(
            "SELECT id, title, author, cover_url, intro, category_id, status, last_update, created_at, click_count, recommend_count FROM novels WHERE title LIKE ?1 OR author LIKE ?1 ORDER BY click_count DESC LIMIT 20",
            [pattern.as_str()],
        )
        .await
        .unwrap();

    let mut list = Vec::new();
    while let Ok(Some(row)) = rows.next().await {
        if let Ok(n) = Novel::from_row(&row) {
            list.push(n);
        }
    }
    Json(list)
}
