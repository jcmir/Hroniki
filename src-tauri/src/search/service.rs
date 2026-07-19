use super::models::{SearchQuery, SearchResult};
use super::repository::SearchRepository;
use std::sync::Arc;

pub struct SearchService {
    repository: Arc<dyn SearchRepository>,
}

impl SearchService {
    pub fn new(repository: Arc<dyn SearchRepository>) -> Self {
        Self { repository }
    }

    pub async fn search(&self, query: SearchQuery) -> Result<Vec<SearchResult>, String> {
        self.repository.search(&query).await
    }

    pub async fn reindex_entry(
        &self,
        entry_id: &str,
        object_id: &str,
        title: &str,
        description: Option<&str>,
        tags: &[String],
    ) -> Result<(), String> {
        self.repository
            .index_entry(entry_id, object_id, title, description, tags)
            .await
    }

    pub async fn remove_entry(&self, entry_id: &str) -> Result<(), String> {
        self.repository.remove_entry(entry_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::super::repository::SqliteSearchRepository;
    use super::*;
    use sqlx::SqlitePool;
    use uuid::Uuid;

    async fn create_test_pool() -> SqlitePool {
        let temp_dir = std::env::temp_dir();
        let db_file = temp_dir.join(format!("hroniki_test_search_{}.sqlite", Uuid::new_v4()));
        let db_url = format!("sqlite://{}", db_file.to_string_lossy().replace('\\', "/"));
        let pool = crate::storage::connection::create_pool(&db_url)
            .await
            .unwrap();
        crate::storage::migrations::run_migrations(&pool)
            .await
            .unwrap();
        pool
    }

    async fn seed_entry(pool: &SqlitePool, title: &str, description: &str) -> String {
        // Insert required category
        let cat_id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO categories (id, name, created_at) VALUES (?, ?, ?)")
            .bind(&cat_id)
            .bind(format!("cat_{}", Uuid::new_v4()))
            .bind(chrono::Utc::now().to_rfc3339())
            .execute(pool)
            .await
            .unwrap();

        // Insert required object
        let obj_id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO objects (id, category_id, name, description, created_at) VALUES (?, ?, ?, ?, ?)")
            .bind(&obj_id)
            .bind(&cat_id)
            .bind("Test Object")
            .bind("desc")
            .bind(chrono::Utc::now().to_rfc3339())
            .execute(pool)
            .await
            .unwrap();

        // Insert entry (triggers will auto-populate entries_fts)
        let entry_id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO entries (id, object_id, occurred_at, title, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(&entry_id)
            .bind(&obj_id)
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(title)
            .bind(description)
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(chrono::Utc::now().to_rfc3339())
            .execute(pool)
            .await
            .unwrap();

        entry_id
    }

    #[tokio::test]
    async fn search_finds_entries_by_title() {
        let pool = create_test_pool().await;
        let repository = Arc::new(SqliteSearchRepository::new(pool.clone()));
        let service = SearchService::new(repository);

        seed_entry(&pool, "Первая посадка яблони", "Высадили саженец в саду").await;
        seed_entry(&pool, "Полив огурцов", "Провели плановый полив").await;

        let results = service
            .search(SearchQuery {
                text: "яблони".to_string(),
                object_id: None,
            })
            .await
            .unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].title.contains("яблони"));

        pool.close().await;
    }

    #[tokio::test]
    async fn search_returns_empty_for_no_match() {
        let pool = create_test_pool().await;
        let repository = Arc::new(SqliteSearchRepository::new(pool.clone()));
        let service = SearchService::new(repository);

        seed_entry(&pool, "Подкормка роз", "Добавили удобрения").await;

        let results = service
            .search(SearchQuery {
                text: "яблоня".to_string(),
                object_id: None,
            })
            .await
            .unwrap();

        assert!(results.is_empty());

        pool.close().await;
    }

    #[tokio::test]
    async fn search_reindex_adds_tags() {
        let pool = create_test_pool().await;
        let repository = Arc::new(SqliteSearchRepository::new(pool.clone()));
        let service = SearchService::new(repository.clone());

        let entry_id = seed_entry(&pool, "Обрезка яблони", "Провели обрезку ветвей").await;

        // Reindex with tags via SearchService
        service
            .reindex_entry(
                &entry_id,
                "some-object-id",
                "Обрезка яблони",
                Some("Провели обрезку ветвей"),
                &["удобрение".to_string(), "уход".to_string()],
            )
            .await
            .unwrap();

        let results = service
            .search(SearchQuery {
                text: "удобрение".to_string(),
                object_id: None,
            })
            .await
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].entry_id, entry_id);

        pool.close().await;
    }
}
