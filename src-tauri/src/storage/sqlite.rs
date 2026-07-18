use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};

use crate::domain::{Category, CategoryId, ChronicleObject, ChronicleObjectId, Entry, EntryId, Photo, PhotoId};

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

    async fn save_entry(&mut self, entry: Entry) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO entries
            (
                id,
                object_id,
                occurred_at,
                title,
                description,
                created_at,
                updated_at
            )
            VALUES
            (
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?
            )
            "#,
        )
        .bind(entry.id.value().to_string())
        .bind(entry.object_id.value().to_string())
        .bind(entry.occurred_at.to_rfc3339())
        .bind(entry.title)
        .bind(entry.description)
        .bind(entry.created_at.to_rfc3339())
        .bind(entry.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
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
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                object_id,
                occurred_at,
                title,
                description,
                created_at,
                updated_at
            FROM entries
            ORDER BY occurred_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|error| error.to_string())?;

        let mut result = Vec::new();

        for row in rows {
            let id: String = row.try_get("id").map_err(|e| e.to_string())?;
            let object_id: String = row.try_get("object_id").map_err(|e| e.to_string())?;
            let occurred_at: String = row.try_get("occurred_at").map_err(|e| e.to_string())?;
            let title: String = row.try_get("title").map_err(|e| e.to_string())?;
            let description: Option<String> =
                row.try_get("description").map_err(|e| e.to_string())?;
            let created_at: String = row.try_get("created_at").map_err(|e| e.to_string())?;
            let updated_at: String = row.try_get("updated_at").map_err(|e| e.to_string())?;

            result.push(Entry {
                id: EntryId::from(uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?),
                object_id: ChronicleObjectId::from(
                    uuid::Uuid::parse_str(&object_id).map_err(|e| e.to_string())?,
                ),
                occurred_at: DateTime::parse_from_rfc3339(&occurred_at)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&Utc),
                title,
                description,
                created_at: DateTime::parse_from_rfc3339(&created_at)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&updated_at)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&Utc),
            });
        }

        Ok(result)
    }

    async fn save_photo(&mut self, photo: Photo) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO photos
            (
                id,
                entry_id,
                path,
                thumbnail,
                created_at
            )
            VALUES
            (
                $1,
                $2,
                $3,
                $4,
                $5
            )
            ON CONFLICT(id) DO UPDATE SET
                entry_id = excluded.entry_id,
                path = excluded.path,
                thumbnail = excluded.thumbnail,
                created_at = excluded.created_at
            "#,
        )
        .bind(photo.id.value().to_string())
        .bind(photo.entry_id.value().to_string())
        .bind(photo.path)
        .bind(photo.thumbnail)
        .bind(photo.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|error| error.to_string())?;

        Ok(())
    }

    async fn entry_photos(&self, entry_id: EntryId) -> Result<Vec<Photo>, String> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                entry_id,
                path,
                thumbnail,
                created_at
            FROM photos
            WHERE entry_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(entry_id.value().to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|error| error.to_string())?;

        let mut result = Vec::new();

        for row in rows {
            let id: String = row.try_get("id").map_err(|e| e.to_string())?;
            let entry_id_str: String = row.try_get("entry_id").map_err(|e| e.to_string())?;
            let path: String = row.try_get("path").map_err(|e| e.to_string())?;
            let thumbnail: String = row.try_get("thumbnail").map_err(|e| e.to_string())?;
            let created_at: String = row.try_get("created_at").map_err(|e| e.to_string())?;

            result.push(Photo {
                id: PhotoId::from(uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?),
                entry_id: EntryId::from(
                    uuid::Uuid::parse_str(&entry_id_str).map_err(|e| e.to_string())?,
                ),
                path,
                thumbnail,
                created_at: DateTime::parse_from_rfc3339(&created_at)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&Utc),
            });
        }

        Ok(result)
    }

    async fn delete_entry(&mut self, id: EntryId) -> Result<(), String> {
        sqlx::query("DELETE FROM entries WHERE id = $1")
            .bind(id.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    async fn update_entry(&mut self, id: EntryId, title: String, description: Option<String>) -> Result<(), String> {
        sqlx::query(
            r#"
            UPDATE entries
            SET
                title = $1,
                description = $2,
                updated_at = $3
            WHERE id = $4
            "#,
        )
        .bind(title)
        .bind(description)
        .bind(Utc::now().to_rfc3339())
        .bind(id.value().to_string())
        .execute(&self.pool)
        .await
        .map_err(|error| error.to_string())?;
        Ok(())
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

    #[tokio::test]
    async fn saves_and_reads_entry() {
        let pool = create_pool("sqlite::memory:").await.unwrap();

        run_migrations(&pool).await.unwrap();

        let mut repository = SqliteChronologyRepository::new(pool);

        let category = Category::new("Garden").unwrap();

        repository.save_category(category.clone()).await.unwrap();

        let object = ChronicleObject::new(category.id, "Apple tree", None).unwrap();

        repository.save_object(object.clone()).await.unwrap();

        let entry = Entry::new(
            object.id,
            Utc::now(),
            "Treatment",
            Some("Fungicide spray".to_string()),
        )
        .unwrap();

        repository.save_entry(entry.clone()).await.unwrap();

        let entries = repository.entries().await.unwrap();

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].title, "Treatment");
        assert_eq!(entries[0].object_id, object.id);
    }

    #[tokio::test]
    async fn saves_and_reads_photo() {
        let pool = create_pool("sqlite::memory:").await.unwrap();

        run_migrations(&pool).await.unwrap();

        let mut repository = SqliteChronologyRepository::new(pool);

        let category = Category::new("Garden").unwrap();
        repository.save_category(category.clone()).await.unwrap();

        let object = ChronicleObject::new(category.id, "Apple tree", None).unwrap();
        repository.save_object(object.clone()).await.unwrap();

        let entry = Entry::new(object.id, Utc::now(), "Treatment", None).unwrap();
        repository.save_entry(entry.clone()).await.unwrap();

        let photo = Photo::new(entry.id, "original.jpg", "thumbnail.jpg");
        repository.save_photo(photo.clone()).await.unwrap();

        let photos = repository.entry_photos(entry.id).await.unwrap();
        assert_eq!(photos.len(), 1);
        assert_eq!(photos[0].path, "original.jpg");
        assert_eq!(photos[0].thumbnail, "thumbnail.jpg");
    }
}
