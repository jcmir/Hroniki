use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};

use crate::domain::{
    Category,
    ChronicleObject,
    Entry,
    CategoryId,
};

use super::ChronologyRepository;

pub struct SqliteChronologyRepository {
    pool: SqlitePool,
}

impl SqliteChronologyRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ChronologyRepository for SqliteChronologyRepository {
    async fn save_category(
        &mut self,
        category: Category,
    ) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO categories
            (
                id,
                name,
                created_at
            )
            VALUES
            (
                ?,
                ?,
                ?
            )
            "#
        )
        .bind(category.id.value().to_string())
        .bind(category.name)
        .bind(category.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|error| error.to_string())?;

        Ok(())
    }

    async fn save_object(
        &mut self,
        _object: ChronicleObject,
    ) -> Result<(), String> {
        todo!()
    }

    async fn save_entry(
        &mut self,
        _entry: Entry,
    ) -> Result<(), String> {
        todo!()
    }

    async fn categories(
        &self,
    ) -> Result<Vec<Category>, String> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                name,
                created_at
            FROM categories
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|error| error.to_string())?;

        let mut result = Vec::new();

        for row in rows {
            let id_str: String = row.get("id");
            let name_str: String = row.get("name");
            let created_at_str: String = row.get("created_at");

            let id = uuid::Uuid::parse_str(&id_str)
                .map_err(|e| e.to_string())?;

            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| e.to_string())?
                .with_timezone(&Utc);

            result.push(Category {
                id: CategoryId::from(id),
                name: name_str,
                created_at,
            });
        }

        Ok(result)
    }

    async fn objects(
        &self,
    ) -> Result<Vec<ChronicleObject>, String> {
        todo!()
    }

    async fn entries(
        &self,
    ) -> Result<Vec<Entry>, String> {
        todo!()
    }
}
