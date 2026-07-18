use serde::{Deserialize, Serialize};
use libsql::Row;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub sort_order: i64,
}

impl Category {
    pub fn from_row(row: &Row) -> Result<Self, libsql::Error> {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            slug: row.get(2)?,
            sort_order: row.get(3)?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Novel {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub cover_url: String,
    pub intro: String,
    pub category_id: i64,
    pub status: String,
    pub last_update: String,
    pub created_at: String,
    pub click_count: i64,
    pub recommend_count: i64,
}

impl Novel {
    pub fn from_row(row: &Row) -> Result<Self, libsql::Error> {
        Ok(Novel {
            id: row.get(0)?,
            title: row.get(1)?,
            author: row.get(2)?,
            cover_url: row.get(3)?,
            intro: row.get(4)?,
            category_id: row.get(5)?,
            status: row.get(6)?,
            last_update: row.get(7)?,
            created_at: row.get(8)?,
            click_count: row.get(9)?,
            recommend_count: row.get(10)?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub id: i64,
    pub novel_id: i64,
    pub title: String,
    pub content: String,
    pub chapter_number: i64,
    pub created_at: String,
}

impl Chapter {
    pub fn from_row(row: &Row) -> Result<Self, libsql::Error> {
        Ok(Chapter {
            id: row.get(0)?,
            novel_id: row.get(1)?,
            title: row.get(2)?,
            content: row.get(3)?,
            chapter_number: row.get(4)?,
            created_at: row.get(5)?,
        })
    }
}

// --- API response types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelWithLatest {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub cover_url: String,
    pub intro: String,
    pub category_id: i64,
    pub status: String,
    pub last_update: String,
    pub created_at: String,
    pub click_count: i64,
    pub recommend_count: i64,
    pub latest_chapter_title: Option<String>,
    pub latest_chapter_id: Option<i64>,
}

impl NovelWithLatest {
    pub fn from_row(row: &Row) -> Result<Self, libsql::Error> {
        Ok(NovelWithLatest {
            id: row.get(0)?,
            title: row.get(1)?,
            author: row.get(2)?,
            cover_url: row.get(3)?,
            intro: row.get(4)?,
            category_id: row.get(5)?,
            status: row.get(6)?,
            last_update: row.get(7)?,
            created_at: row.get(8)?,
            click_count: row.get(9)?,
            recommend_count: row.get(10)?,
            latest_chapter_title: row.get(11).ok(),
            latest_chapter_id: row.get(12).ok(),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct CategoryBlock {
    pub category: Category,
    pub featured_novel: Novel,
    pub novels: Vec<Novel>,
}

#[derive(Debug, Serialize)]
pub struct HomeResponse {
    pub featured: Vec<Novel>,
    pub weekly_ranking: Vec<Novel>,
    pub recommend: Vec<Novel>,
    pub category_blocks: Vec<CategoryBlock>,
    pub latest_updates: Vec<NovelWithLatest>,
    pub new_arrivals: Vec<Novel>,
}

#[derive(Debug, Serialize)]
pub struct NovelDetail {
    pub novel: Novel,
    pub category_name: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Serialize)]
pub struct ChapterResponse {
    pub novel: Novel,
    pub category_name: String,
    pub chapter: Chapter,
    pub prev_chapter_id: Option<i64>,
    pub next_chapter_id: Option<i64>,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Serialize)]
pub struct NovelListResponse {
    pub novels: Vec<NovelWithLatest>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub category_name: Option<String>,
}
