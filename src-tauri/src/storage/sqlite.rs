use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};

use crate::domain::{Category, CategoryId, ChronicleObject, ChronicleObjectId, Entry};

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
    async fn save_category(&mut self, category: Category) -> Result<(), String> {
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
            "#,
        )
        .bind(category.id.value().to_string())
        .bind(category.name)
        .bind(category.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|error| error.to_string())?;

        Ok(())
    }

    async fn save_object(&mut self, object: ChronicleObject) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO objects
            (
                id,
                category_id,
                name,
                description,
                created_at
            )
            VALUES
            (
                ?,
                ?,
                ?,
                ?,
                ?
            )
            "#,
        )
        .bind(object.id.value().to_string())
        .bind(object.category_id.value().to_string())
        .bind(object.name)
        .bind(object.description)
        .bind(object.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn save_entry(&mut self, _entry: Entry) -> Result<(), String> {
        todo!()
    }

    async fn categories(&self) -> Result<Vec<Category>, String> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                name,
                created_at
            FROM categories
            ORDER BY created_at
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|error| error.to_string())?;

        let mut result = Vec::new();

        for row in rows {
            let id: String = row.try_get("id").map_err(|e| e.to_string())?;
            let name: String = row.try_get("name").map_err(|e| e.to_string())?;
            let created_at: String = row.try_get("created_at").map_err(|e| e.to_string())?;

            result.push(Category {
                id: CategoryId::from(uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?),
                name,
                created_at: DateTime::parse_from_rfc3339(&created_at)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&Utc),
            });
        }

        Ok(result)
    }

    async fn objects(&self) -> Result<Vec<ChronicleObject>, String> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                category_id,
                name,
                description,
                created_at
            FROM objects
            ORDER BY created_at
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|error| error.to_string())?;

        let mut result = Vec::new();

        for row in rows {
            let id: String = row.try_get("id").map_err(|e| e.to_string())?;
            let category_id: String = row.try_get("category_id").map_err(|e| e.to_string())?;
            let name: String = row.try_get("name").map_err(|e| e.to_string())?;
            let description: Option<String> =
                row.try_get("description").map_err(|e| e.to_string())?;
            let created_at: String = row.try_get("created_at").map_err(|e| e.to_string())?;

            result.push(ChronicleObject {
                id: ChronicleObjectId::from(uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?),
                category_id: CategoryId::from(
                    uuid::Uuid::parse_str(&category_id).map_err(|e| e.to_string())?,
                ),
                name,
                description,
                created_at: DateTime::parse_from_rfc3339(&created_at)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&Utc),
            });
        }

        Ok(result)
    }

    async fn entries(&self) -> Result<Vec<Entry>, String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Category;
    use crate::storage::connection::create_pool;
    use crate::storage::migrations::run_migrations;

    #[tokio::test]
    async fn saves_and_reads_category() {
        let pool = create_pool("sqlite::memory:").await.unwrap();

        run_migrations(&pool).await.unwrap();

        let mut repository = SqliteChronologyRepository::new(pool);

        let category = Category::new("Garden").unwrap();

        repository.save_category(category.clone()).await.unwrap();

        let categories = repository.categories().await.unwrap();

        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].name, "Garden");
    }

    #[tokio::test]
    async fn saves_and_reads_object() {
        let pool = create_pool("sqlite::memory:").await.unwrap();

        run_migrations(&pool).await.unwrap();

        let mut repository = SqliteChronologyRepository::new(pool);

        let category = Category::new("Garden").unwrap();

        repository.save_category(category.clone()).await.unwrap();

        let object = ChronicleObject::new(category.id, "Apple tree", None).unwrap();

        repository.save_object(object.clone()).await.unwrap();

        let objects = repository.objects().await.unwrap();

        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].name, "Apple tree");
    }
}
