use axum::{extract::State, Json};
use libsql::Database;
use std::sync::Arc;

use crate::models::{Category, CategoryBlock, HomeResponse, Novel, NovelWithLatest};

pub async fn home_data(State(db): State<Arc<Database>>) -> Json<HomeResponse> {
    let conn = db.connect().unwrap();

    let featured = query_novels(&conn, "SELECT id, title, author, cover_url, intro, category_id, status, last_update, created_at, click_count, recommend_count FROM novels ORDER BY click_count DESC LIMIT 4").await;
    let weekly_ranking = query_novels(&conn, "SELECT id, title, author, cover_url, intro, category_id, status, last_update, created_at, click_count, recommend_count FROM novels ORDER BY recommend_count DESC LIMIT 6").await;
    let recommend = query_novels(&conn, "SELECT id, title, author, cover_url, intro, category_id, status, last_update, created_at, click_count, recommend_count FROM novels ORDER BY click_count DESC LIMIT 5").await;
    let categories = query_categories(&conn).await;

    let mut category_blocks = Vec::new();
    for cat in categories.iter().take(6) {
        let sql = format!("SELECT id, title, author, cover_url, intro, category_id, status, last_update, created_at, click_count, recommend_count FROM novels WHERE category_id = {} ORDER BY click_count DESC LIMIT 1", cat.id);
        let featured_novel = query_one_novel(&conn, &sql).await;

        let sql2 = format!("SELECT id, title, author, cover_url, intro, category_id, status, last_update, created_at, click_count, recommend_count FROM novels WHERE category_id = {} ORDER BY click_count DESC LIMIT 6", cat.id);
        let novels = query_novels(&conn, &sql2).await;

        if let Some(fn2) = featured_novel {
            category_blocks.push(CategoryBlock {
                category: cat.clone(),
                featured_novel: fn2,
                novels,
            });
        }
    }

    let latest_updates = query_novels_with_latest(&conn,
        "SELECT n.id, n.title, n.author, n.cover_url, n.intro, n.category_id, n.status, n.last_update, n.created_at, n.click_count, n.recommend_count, c.title as lt, c.id as lc FROM novels n LEFT JOIN chapters c ON c.novel_id = n.id AND c.chapter_number = (SELECT MAX(chapter_number) FROM chapters WHERE novel_id = n.id) ORDER BY n.last_update DESC LIMIT 10"
    ).await;

    let new_arrivals = query_novels(&conn, "SELECT id, title, author, cover_url, intro, category_id, status, last_update, created_at, click_count, recommend_count FROM novels ORDER BY created_at DESC LIMIT 6").await;

    Json(HomeResponse { featured, weekly_ranking, recommend, category_blocks, latest_updates, new_arrivals })
}

async fn query_novels(conn: &libsql::Connection, sql: &str) -> Vec<Novel> {
    let mut rows = conn.query(sql, ()).await.unwrap();
    let mut list = Vec::new();
    while let Ok(Some(row)) = rows.next().await {
        if let Ok(n) = Novel::from_row(&row) { list.push(n); }
    }
    list
}

async fn query_one_novel(conn: &libsql::Connection, sql: &str) -> Option<Novel> {
    let mut rows = conn.query(sql, ()).await.ok()?;
    if let Ok(Some(row)) = rows.next().await {
        Novel::from_row(&row).ok()
    } else { None }
}

async fn query_categories(conn: &libsql::Connection) -> Vec<Category> {
    let mut rows = conn.query("SELECT id, name, slug, sort_order FROM categories ORDER BY sort_order", ()).await.unwrap();
    let mut list = Vec::new();
    while let Ok(Some(row)) = rows.next().await {
        if let Ok(c) = Category::from_row(&row) { list.push(c); }
    }
    list
}

async fn query_novels_with_latest(conn: &libsql::Connection, sql: &str) -> Vec<NovelWithLatest> {
    let mut rows = conn.query(sql, ()).await.unwrap();
    let mut list = Vec::new();
    while let Ok(Some(row)) = rows.next().await {
        if let Ok(n) = NovelWithLatest::from_row(&row) { list.push(n); }
    }
    list
}
