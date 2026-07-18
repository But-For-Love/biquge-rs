use crate::types::*;
use gloo_net::http::Request;

const BASE: &str = "http://localhost:3000/api";

pub async fn fetch_home() -> Result<HomeResponse, String> {
    Request::get(&format!("{}/home", BASE))
        .send().await
        .map_err(|e| e.to_string())?
        .json().await
        .map_err(|e| e.to_string())
}

pub async fn fetch_categories() -> Result<Vec<Category>, String> {
    Request::get(&format!("{}/categories", BASE))
        .send().await
        .map_err(|e| e.to_string())?
        .json().await
        .map_err(|e| e.to_string())
}

pub async fn fetch_novels(category_id: Option<i64>, page: i64, sort: &str) -> Result<NovelListResponse, String> {
    let mut url = format!("{}/novels?page={}&sort={}", BASE, page, sort);
    if let Some(cid) = category_id {
        url.push_str(&format!("&category_id={}", cid));
    }
    Request::get(&url)
        .send().await
        .map_err(|e| e.to_string())?
        .json().await
        .map_err(|e| e.to_string())
}

pub async fn fetch_novel_detail(id: i64) -> Result<Option<NovelDetail>, String> {
    Request::get(&format!("{}/novels/{}", BASE, id))
        .send().await
        .map_err(|e| e.to_string())?
        .json().await
        .map_err(|e| e.to_string())
}

pub async fn fetch_chapter(novel_id: i64, chapter_id: i64) -> Result<Option<ChapterResponse>, String> {
    Request::get(&format!("{}/chapters/{}/{}", BASE, novel_id, chapter_id))
        .send().await
        .map_err(|e| e.to_string())?
        .json().await
        .map_err(|e| e.to_string())
}

pub async fn fetch_search(query: &str) -> Result<Vec<Novel>, String> {
    Request::get(&format!("{}/search?q={}", BASE, urlencoding(query)))
        .send().await
        .map_err(|e| e.to_string())?
        .json().await
        .map_err(|e| e.to_string())
}

pub async fn fetch_rankings(rtype: &str) -> Result<Vec<Novel>, String> {
    Request::get(&format!("{}/rankings?type={}", BASE, rtype))
        .send().await
        .map_err(|e| e.to_string())?
        .json().await
        .map_err(|e| e.to_string())
}

fn urlencoding(s: &str) -> String {
    // Simple URL encoding — replace spaces with +
    s.replace(' ', "+")
}
