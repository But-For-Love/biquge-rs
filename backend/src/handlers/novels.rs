use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use libsql::Database;
use std::sync::Arc;

use crate::models::{Chapter, Novel, NovelDetail, NovelListResponse, NovelWithLatest};

#[derive(Deserialize)]
pub struct ListQuery {
    pub category_id: Option<i64>,
    pub page: Option<i64>,
    pub sort: Option<String>,
    pub fullflag: Option<i64>,
}

pub async fn list(
    State(db): State<Arc<Database>>,
    Query(params): Query<ListQuery>,
) -> Json<NovelListResponse> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = 20i64;
    let offset = (page - 1) * page_size;

    let order = match params.sort.as_deref() {
        Some("new") => "n.created_at DESC",
        Some("click") => "n.click_count DESC",
        Some("recommend") => "n.recommend_count DESC",
        _ => "n.click_count DESC",
    };

    let conn = db.connect().unwrap();

    let mut where_clause = String::new();
    if let Some(cid) = params.category_id {
        where_clause = format!(" WHERE n.category_id = {}", cid);
    } else if let Some(ff) = params.fullflag {
        let status = if ff == 1 { "completed" } else { "ongoing" };
        where_clause = format!(" WHERE n.status = '{}'", status);
    }

    let total = {
        let sql = format!("SELECT COUNT(*) as cnt FROM novels n{}", where_clause);
        let mut rows = conn.query(&sql, ()).await.unwrap();
        if let Ok(Some(row)) = rows.next().await { row.get(0).unwrap_or(0) } else { 0 }
    };

    let list_sql = format!(
        "SELECT n.id, n.title, n.author, n.cover_url, n.intro, n.category_id, n.status, n.last_update, n.created_at, n.click_count, n.recommend_count, c.title as lt, c.id as lc FROM novels n LEFT JOIN chapters c ON c.novel_id = n.id AND c.chapter_number = (SELECT MAX(chapter_number) FROM chapters WHERE novel_id = n.id) {} ORDER BY {} LIMIT {} OFFSET {}",
        where_clause, order, page_size, offset
    );

    let mut rows = conn.query(&list_sql, ()).await.unwrap();
    let mut novels = Vec::new();
    while let Ok(Some(row)) = rows.next().await {
        if let Ok(n) = NovelWithLatest::from_row(&row) { novels.push(n); }
    }

    let category_name = if let Some(cid) = params.category_id {
        let sql = format!("SELECT name FROM categories WHERE id = {}", cid);
        let mut rows = conn.query(&sql, ()).await.unwrap();
        if let Ok(Some(row)) = rows.next().await {
            Some(row.get::<String>(0).unwrap_or_default())
        } else { None }
    } else { None };

    Json(NovelListResponse { novels, total, page, page_size, category_name })
}

pub async fn detail(
    State(db): State<Arc<Database>>,
    Path(id): Path<i64>,
) -> Json<Option<NovelDetail>> {
    let conn = db.connect().unwrap();

    let sql = format!("SELECT id, title, author, cover_url, intro, category_id, status, last_update, created_at, click_count, recommend_count FROM novels WHERE id = {}", id);
    let mut rows = conn.query(&sql, ()).await.unwrap();
    let novel = if let Ok(Some(row)) = rows.next().await {
        Novel::from_row(&row).ok()
    } else { None };

    match novel {
        Some(novel) => {
            let cat_sql = format!("SELECT name FROM categories WHERE id = {}", novel.category_id);
            let mut rows = conn.query(&cat_sql, ()).await.unwrap();
            let category_name = if let Ok(Some(row)) = rows.next().await {
                row.get::<String>(0).unwrap_or_else(|_| "未知".to_string())
            } else { "未知".to_string() };

            let ch_sql = format!("SELECT id, novel_id, title, content, chapter_number, created_at FROM chapters WHERE novel_id = {} ORDER BY chapter_number", novel.id);
            let mut rows = conn.query(&ch_sql, ()).await.unwrap();
            let mut chapters = Vec::new();
            while let Ok(Some(row)) = rows.next().await {
                if let Ok(c) = Chapter::from_row(&row) { chapters.push(c); }
            }

            Json(Some(NovelDetail { novel, category_name, chapters }))
        }
        None => Json(None),
    }
}
