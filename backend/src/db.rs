use libsql::{Builder, Database};
use std::path::Path;
use std::sync::Arc;

pub async fn init_db(db: &Arc<Database>) {
    let conn = db.connect().unwrap();

    let sql = r#"
        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            slug TEXT NOT NULL UNIQUE,
            sort_order INTEGER DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS novels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            cover_url TEXT DEFAULT '',
            intro TEXT DEFAULT '',
            category_id INTEGER NOT NULL,
            status TEXT DEFAULT 'ongoing',
            last_update TEXT DEFAULT '',
            created_at TEXT DEFAULT '',
            click_count INTEGER DEFAULT 0,
            recommend_count INTEGER DEFAULT 0,
            FOREIGN KEY (category_id) REFERENCES categories(id)
        );

        CREATE TABLE IF NOT EXISTS chapters (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            novel_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            chapter_number INTEGER NOT NULL,
            created_at TEXT DEFAULT '',
            FOREIGN KEY (novel_id) REFERENCES novels(id)
        );

        CREATE INDEX IF NOT EXISTS idx_novels_category ON novels(category_id);
        CREATE INDEX IF NOT EXISTS idx_novels_click ON novels(click_count DESC);
        CREATE INDEX IF NOT EXISTS idx_novels_created ON novels(created_at DESC);
        CREATE INDEX IF NOT EXISTS idx_chapters_novel ON chapters(novel_id);
    "#;

    conn.execute_batch(sql).await.expect("Failed to create tables");
}

pub async fn connect(db_path: &str) -> Database {
    if let Some(parent) = Path::new(db_path).parent() {
        std::fs::create_dir_all(parent).ok();
    }

    Builder::new_local(db_path)
        .build()
        .await
        .expect("Failed to open database")
}
