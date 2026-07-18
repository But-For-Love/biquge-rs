use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub sort_order: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    pub id: i64,
    pub novel_id: i64,
    pub title: String,
    pub content: String,
    pub chapter_number: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategoryBlock {
    pub category: Category,
    pub featured_novel: Novel,
    pub novels: Vec<Novel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomeResponse {
    pub featured: Vec<Novel>,
    pub weekly_ranking: Vec<Novel>,
    pub recommend: Vec<Novel>,
    pub category_blocks: Vec<CategoryBlock>,
    pub latest_updates: Vec<NovelWithLatest>,
    pub new_arrivals: Vec<Novel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovelDetail {
    pub novel: Novel,
    pub category_name: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChapterResponse {
    pub novel: Novel,
    pub category_name: String,
    pub chapter: Chapter,
    pub prev_chapter_id: Option<i64>,
    pub next_chapter_id: Option<i64>,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovelListResponse {
    pub novels: Vec<NovelWithLatest>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub category_name: Option<String>,
}
