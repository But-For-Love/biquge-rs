use axum::{extract::{Query, State}, Json};
use serde::Deserialize;
use libsql::Database;
use std::sync::Arc;

use crate::models::Novel;

#[derive(Deserialize)]
pub struct RankingsQuery {
    #[serde(rename = "type")]
    pub rtype: Option<String>,
}

pub async fn list(
    State(db): State<Arc<Database>>,
    Query(params): Query<RankingsQuery>,
) -> Json<Vec<Novel>> {
    let order = match params.rtype.as_deref() {
        Some("recommend") => "recommend_count DESC",
        Some("new") => "created_at DESC",
        _ => "click_count DESC",
    };
    let sql = format!(
        "SELECT id, title, author, cover_url, intro, category_id, status, last_update, created_at, click_count, recommend_count FROM novels ORDER BY {} LIMIT 20",
        order
    );

    let conn = db.connect().unwrap();
    let mut rows = conn.query(&sql, ()).await.unwrap();

    let mut list = Vec::new();
    while let Ok(Some(row)) = rows.next().await {
        if let Ok(n) = Novel::from_row(&row) {
            list.push(n);
        }
    }
    Json(list)
}
