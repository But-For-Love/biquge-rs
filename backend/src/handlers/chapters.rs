use axum::{extract::{Path, State}, Json};
use libsql::Database;
use std::sync::Arc;

use crate::models::{Chapter, ChapterResponse, Novel};

pub async fn read(
    State(db): State<Arc<Database>>,
    Path((novel_id, chapter_id)): Path<(i64, i64)>,
) -> Json<Option<ChapterResponse>> {
    let conn = db.connect().unwrap();

    // Fetch novel
    let novel_sql = format!("SELECT id, title, author, cover_url, intro, category_id, status, last_update, created_at, click_count, recommend_count FROM novels WHERE id = {}", novel_id);
    let mut rows = conn.query(&novel_sql, ()).await.unwrap();
    let novel = match rows.next().await {
        Ok(Some(row)) => match Novel::from_row(&row) {
            Ok(n) => n,
            Err(_) => return Json(None),
        },
        _ => return Json(None),
    };

    // Category name
    let cat_sql = format!("SELECT name FROM categories WHERE id = {}", novel.category_id);
    let category_name = {
        let mut rows = conn.query(&cat_sql, ()).await.unwrap();
        if let Ok(Some(row)) = rows.next().await {
            row.get::<String>(0).unwrap_or_else(|_| "未知".to_string())
        } else { "未知".to_string() }
    };

    // Fetch chapter
    let ch_sql = format!("SELECT id, novel_id, title, content, chapter_number, created_at FROM chapters WHERE novel_id = {} AND id = {}", novel_id, chapter_id);
    let mut rows = conn.query(&ch_sql, ()).await.unwrap();
    let chapter = match rows.next().await {
        Ok(Some(row)) => match Chapter::from_row(&row) {
            Ok(c) => c,
            Err(_) => return Json(None),
        },
        _ => return Json(None),
    };

    // Prev/next
    let prev_sql = format!("SELECT id FROM chapters WHERE novel_id = {} AND chapter_number < {} ORDER BY chapter_number DESC LIMIT 1", novel_id, chapter.chapter_number);
    let prev_id = {
        let mut rows = conn.query(&prev_sql, ()).await.unwrap();
        if let Ok(Some(row)) = rows.next().await { row.get::<i64>(0).ok() } else { None }
    };

    let next_sql = format!("SELECT id FROM chapters WHERE novel_id = {} AND chapter_number > {} ORDER BY chapter_number ASC LIMIT 1", novel_id, chapter.chapter_number);
    let next_id = {
        let mut rows = conn.query(&next_sql, ()).await.unwrap();
        if let Ok(Some(row)) = rows.next().await { row.get::<i64>(0).ok() } else { None }
    };

    // All chapters (without content — only reader needs content)
    let all_ch_sql = format!("SELECT id, novel_id, title, content, chapter_number, created_at FROM chapters WHERE novel_id = {} ORDER BY chapter_number", novel_id);
    let mut rows = conn.query(&all_ch_sql, ()).await.unwrap();
    let mut chapters = Vec::new();
    while let Ok(Some(row)) = rows.next().await {
        if let Ok(c) = Chapter::from_row(&row) { chapters.push(c); }
    }

    Json(Some(ChapterResponse { novel, category_name, chapter, prev_chapter_id: prev_id, next_chapter_id: next_id, chapters }))
}
