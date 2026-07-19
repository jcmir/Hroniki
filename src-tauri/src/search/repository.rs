use super::models::{SearchQuery, SearchResult};
use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

#[async_trait]
pub trait SearchRepository: Send + Sync {
    async fn index_entry(
        &self,
        entry_id: &str,
        object_id: &str,
        title: &str,
        description: Option<&str>,
        tags: &[String],
    ) -> Result<(), String>;

    async fn remove_entry(&self, entry_id: &str) -> Result<(), String>;

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, String>;
}

pub struct SqliteSearchRepository {
    pool: SqlitePool,
}

impl SqliteSearchRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SearchRepository for SqliteSearchRepository {
    async fn index_entry(
        &self,
        entry_id: &str,
        object_id: &str,
        title: &str,
        description: Option<&str>,
        tags: &[String],
    ) -> Result<(), String> {
        let tags_str = tags.join(" ");
        let desc = description.unwrap_or("");

        // Upsert: delete existing then re-insert to update tags
        sqlx::query("DELETE FROM entries_fts WHERE entry_id = ?")
            .bind(entry_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO entries_fts(entry_id, object_id, title, description, tags) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(entry_id)
        .bind(object_id)
        .bind(title)
        .bind(desc)
        .bind(&tags_str)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn remove_entry(&self, entry_id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM entries_fts WHERE entry_id = ?")
            .bind(entry_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, String> {
        // Sanitize the FTS5 query: escape special chars to avoid syntax errors
        let fts_query = query.text.trim().to_string();
        if fts_query.is_empty() {
            return Ok(vec![]);
        }

        let rows = if let Some(ref oid) = query.object_id {
            sqlx::query(
                r#"
                SELECT
                    entry_id,
                    object_id,
                    title,
                    snippet(entries_fts, 3, '<b>', '</b>', '...', 20) AS snippet
                FROM entries_fts
                WHERE entries_fts MATCH ?
                  AND object_id = ?
                ORDER BY rank
                LIMIT 50
                "#,
            )
            .bind(&fts_query)
            .bind(oid)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?
        } else {
            sqlx::query(
                r#"
                SELECT
                    entry_id,
                    object_id,
                    title,
                    snippet(entries_fts, 3, '<b>', '</b>', '...', 20) AS snippet
                FROM entries_fts
                WHERE entries_fts MATCH ?
                ORDER BY rank
                LIMIT 50
                "#,
            )
            .bind(&fts_query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?
        };

        let mut results = Vec::new();
        for r in rows {
            results.push(SearchResult {
                entry_id: r.try_get("entry_id").map_err(|e| e.to_string())?,
                object_id: r.try_get("object_id").map_err(|e| e.to_string())?,
                title: r.try_get("title").map_err(|e| e.to_string())?,
                snippet: r.try_get("snippet").map_err(|e| e.to_string())?,
            });
        }

        Ok(results)
    }
}
