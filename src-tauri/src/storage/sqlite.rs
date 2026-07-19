use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};

use crate::domain::{
    Category, CategoryId, ChronicleObject, ChronicleObjectId, Entry, EntryId, Photo, PhotoId,
    Reminder, ReminderId,
};

use super::{ChronologyRepository, ObjectStats};

pub struct SqliteChronologyRepository {
    pool: SqlitePool,
}

impl SqliteChronologyRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
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

    async fn save_entry_with_photos(
        &mut self,
        entry: Entry,
        photos: Vec<Photo>,
    ) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

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
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        for photo in photos {
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
                    ?,
                    ?,
                    ?,
                    ?,
                    ?
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
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
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

    async fn update_entry(
        &mut self,
        id: EntryId,
        title: String,
        description: Option<String>,
    ) -> Result<(), String> {
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

    async fn save_reminder(&mut self, reminder: Reminder) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO reminders
            (
                id,
                entry_id,
                trigger_at,
                status,
                repeat_days,
                completed_at
            )
            VALUES
            (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6
            )
            ON CONFLICT(id) DO UPDATE SET
                entry_id = excluded.entry_id,
                trigger_at = excluded.trigger_at,
                status = excluded.status,
                repeat_days = excluded.repeat_days,
                completed_at = excluded.completed_at
            "#,
        )
        .bind(reminder.id.value().to_string())
        .bind(reminder.entry_id.value().to_string())
        .bind(reminder.trigger_at.to_rfc3339())
        .bind(reminder.status)
        .bind(reminder.repeat_days)
        .bind(reminder.completed_at.map(|dt| dt.to_rfc3339()))
        .execute(&self.pool)
        .await
        .map_err(|error| error.to_string())?;

        Ok(())
    }

    async fn entry_reminders(&self, entry_id: EntryId) -> Result<Vec<Reminder>, String> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                entry_id,
                trigger_at,
                status,
                repeat_days,
                completed_at
            FROM reminders
            WHERE entry_id = $1
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
            let trigger_at: String = row.try_get("trigger_at").map_err(|e| e.to_string())?;
            let status: String = row.try_get("status").map_err(|e| e.to_string())?;
            let repeat_days: Option<i32> = row.try_get("repeat_days").map_err(|e| e.to_string())?;
            let completed_at_str: Option<String> =
                row.try_get("completed_at").map_err(|e| e.to_string())?;

            let completed_at = match completed_at_str {
                Some(s) => Some(
                    DateTime::parse_from_rfc3339(&s)
                        .map_err(|e| e.to_string())?
                        .with_timezone(&Utc),
                ),
                None => None,
            };

            result.push(Reminder {
                id: ReminderId::from(uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?),
                entry_id: EntryId::from(
                    uuid::Uuid::parse_str(&entry_id_str).map_err(|e| e.to_string())?,
                ),
                trigger_at: DateTime::parse_from_rfc3339(&trigger_at)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&Utc),
                status,
                repeat_days,
                completed_at,
            });
        }

        Ok(result)
    }

    async fn reminders(&self) -> Result<Vec<Reminder>, String> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                entry_id,
                trigger_at,
                status,
                repeat_days,
                completed_at
            FROM reminders
            ORDER BY trigger_at ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|error| error.to_string())?;

        let mut result = Vec::new();

        for row in rows {
            let id: String = row.try_get("id").map_err(|e| e.to_string())?;
            let entry_id_str: String = row.try_get("entry_id").map_err(|e| e.to_string())?;
            let trigger_at: String = row.try_get("trigger_at").map_err(|e| e.to_string())?;
            let status: String = row.try_get("status").map_err(|e| e.to_string())?;
            let repeat_days: Option<i32> = row.try_get("repeat_days").map_err(|e| e.to_string())?;
            let completed_at_str: Option<String> =
                row.try_get("completed_at").map_err(|e| e.to_string())?;

            let completed_at = match completed_at_str {
                Some(s) => Some(
                    DateTime::parse_from_rfc3339(&s)
                        .map_err(|e| e.to_string())?
                        .with_timezone(&Utc),
                ),
                None => None,
            };

            result.push(Reminder {
                id: ReminderId::from(uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?),
                entry_id: EntryId::from(
                    uuid::Uuid::parse_str(&entry_id_str).map_err(|e| e.to_string())?,
                ),
                trigger_at: DateTime::parse_from_rfc3339(&trigger_at)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&Utc),
                status,
                repeat_days,
                completed_at,
            });
        }

        Ok(result)
    }

    async fn search_entries(
        &self,
        query_text: Option<String>,
        category_id: Option<String>,
        object_id: Option<String>,
        start_date: Option<String>,
        end_date: Option<String>,
    ) -> Result<Vec<Entry>, String> {
        let text_filter = query_text.map(|q| format!("%{}%", q));

        let rows = sqlx::query(
            r#"
            SELECT
                e.id,
                e.object_id,
                e.occurred_at,
                e.title,
                e.description,
                e.created_at,
                e.updated_at
            FROM entries e
            JOIN objects o ON e.object_id = o.id
            WHERE (?1 IS NULL OR LOWER(e.title) LIKE LOWER(?1) OR LOWER(e.description) LIKE LOWER(?1))
              AND (?2 IS NULL OR o.category_id = ?2)
              AND (?3 IS NULL OR e.object_id = ?3)
              AND (?4 IS NULL OR e.occurred_at >= ?4)
              AND (?5 IS NULL OR e.occurred_at <= ?5)
            ORDER BY e.occurred_at DESC
            "#,
        )
        .bind(text_filter)
        .bind(category_id)
        .bind(object_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|error| error.to_string())?;

        let mut result = Vec::new();

        for row in rows {
            let id: String = row.try_get("id").map_err(|e| e.to_string())?;
            let object_id_str: String = row.try_get("object_id").map_err(|e| e.to_string())?;
            let occurred_at: String = row.try_get("occurred_at").map_err(|e| e.to_string())?;
            let title: String = row.try_get("title").map_err(|e| e.to_string())?;
            let description: Option<String> =
                row.try_get("description").map_err(|e| e.to_string())?;
            let created_at: String = row.try_get("created_at").map_err(|e| e.to_string())?;
            let updated_at: String = row.try_get("updated_at").map_err(|e| e.to_string())?;

            result.push(Entry {
                id: EntryId::from(uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?),
                object_id: ChronicleObjectId::from(
                    uuid::Uuid::parse_str(&object_id_str).map_err(|e| e.to_string())?,
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

    async fn get_object_stats(
        &self,
        object_id: crate::domain::ChronicleObjectId,
    ) -> Result<ObjectStats, String> {
        let object_id_str = object_id.value().to_string();

        let row = sqlx::query(
            r#"
            SELECT
                o.created_at AS created_at,
                (SELECT COUNT(*) FROM entries WHERE object_id = o.id) AS total_entries,
                (SELECT COUNT(*) FROM photos p JOIN entries e ON p.entry_id = e.id WHERE e.object_id = o.id) AS total_photos,
                (SELECT title FROM entries WHERE object_id = o.id ORDER BY occurred_at DESC LIMIT 1) AS last_event_title,
                (SELECT occurred_at FROM entries WHERE object_id = o.id ORDER BY occurred_at DESC LIMIT 1) AS last_event_date,
                (SELECT MIN(r.trigger_at) FROM reminders r JOIN entries e ON r.entry_id = e.id WHERE e.object_id = o.id AND r.status = 'Scheduled') AS next_reminder_date
            FROM objects o
            WHERE o.id = ?
            "#
        )
        .bind(&object_id_str)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let created_at_str: String = row.try_get("created_at").map_err(|e| e.to_string())?;
        let total_entries: i64 = row.try_get("total_entries").map_err(|e| e.to_string())?;
        let total_photos: i64 = row.try_get("total_photos").map_err(|e| e.to_string())?;
        let last_event_title: Option<String> =
            row.try_get("last_event_title").map_err(|e| e.to_string())?;
        let last_event_date: Option<String> =
            row.try_get("last_event_date").map_err(|e| e.to_string())?;
        let next_reminder_date: Option<String> = row
            .try_get("next_reminder_date")
            .map_err(|e| e.to_string())?;

        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| e.to_string())?
            .with_timezone(&Utc);

        let age_days = (Utc::now() - created_at).num_days();

        Ok(ObjectStats {
            age_days: if age_days < 0 { 0 } else { age_days },
            total_entries: total_entries as usize,
            total_photos: total_photos as usize,
            last_event_title,
            last_event_date,
            next_reminder_date,
        })
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

    #[tokio::test]
    async fn saves_and_reads_reminder() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        run_migrations(&pool).await.unwrap();

        let mut repository = SqliteChronologyRepository::new(pool);

        let category = Category::new("Garden").unwrap();
        repository.save_category(category.clone()).await.unwrap();

        let object = ChronicleObject::new(category.id, "Apple tree", None).unwrap();
        repository.save_object(object.clone()).await.unwrap();

        let entry = Entry::new(object.id, Utc::now(), "Treatment", None).unwrap();
        repository.save_entry(entry.clone()).await.unwrap();

        let trigger = Utc::now() + chrono::Duration::days(14);
        let reminder = Reminder::new(entry.id.clone(), trigger, Some(14));
        repository.save_reminder(reminder.clone()).await.unwrap();

        // Load all reminders
        let reminders = repository.reminders().await.unwrap();
        assert_eq!(reminders.len(), 1);
        assert_eq!(reminders[0].status, "Scheduled");
        assert_eq!(reminders[0].repeat_days, Some(14));

        // Load via entry
        let entry_reminders = repository.entry_reminders(entry.id).await.unwrap();
        assert_eq!(entry_reminders.len(), 1);

        // Update to Completed
        let mut completed = reminder.clone();
        completed.status = "Completed".to_string();
        completed.completed_at = Some(Utc::now());
        repository.save_reminder(completed).await.unwrap();

        let after = repository.reminders().await.unwrap();
        assert_eq!(after.len(), 1);
        assert_eq!(after[0].status, "Completed");
    }

    #[tokio::test]
    async fn foreign_key_violating_object_fails() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        run_migrations(&pool).await.unwrap();
        let mut repository = SqliteChronologyRepository::new(pool);

        let non_existent_cat_id = crate::domain::CategoryId::from(uuid::Uuid::new_v4());
        let object = ChronicleObject::new(non_existent_cat_id, "Ghost tree", None).unwrap();

        let result = repository.save_object(object).await;
        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(
            err_msg.contains("FOREIGN KEY constraint failed")
                || err_msg.contains("foreign key constraint failed")
        );
    }

    #[tokio::test]
    async fn save_entry_with_photos_rolls_back_on_failure() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        run_migrations(&pool).await.unwrap();
        let mut repository = SqliteChronologyRepository::new(pool);

        let category = Category::new("Garden").unwrap();
        repository.save_category(category.clone()).await.unwrap();

        let object = ChronicleObject::new(category.id, "Apple tree", None).unwrap();
        repository.save_object(object.clone()).await.unwrap();

        let entry = Entry::new(object.id, Utc::now(), "Treatment", None).unwrap();

        // Create a photo that violates foreign key constraint (non-existent entry id)
        let invalid_entry_id = crate::domain::EntryId::from(uuid::Uuid::new_v4());
        let invalid_photo = Photo::new(invalid_entry_id, "ghost.jpg", "ghost.jpg");

        // Try to save both - this should fail since the photo has an invalid entry id
        let result = repository
            .save_entry_with_photos(entry.clone(), vec![invalid_photo])
            .await;
        assert!(result.is_err());

        // Verify that the entry WAS NOT saved (transaction rolled back)
        let entries = repository.entries().await.unwrap();
        assert!(entries.is_empty());
    }

    #[tokio::test]
    async fn test_reminder_scheduler_claim_logic() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        run_migrations(&pool).await.unwrap();
        let mut repository = SqliteChronologyRepository::new(pool.clone());

        let category = Category::new("Garden").unwrap();
        repository.save_category(category.clone()).await.unwrap();

        let object = ChronicleObject::new(category.id, "Apple tree", None).unwrap();
        repository.save_object(object.clone()).await.unwrap();

        let entry = Entry::new(object.id, Utc::now(), "Treatment", None).unwrap();
        repository.save_entry(entry.clone()).await.unwrap();

        let trigger = Utc::now() - chrono::Duration::seconds(10); // in the past
        let reminder = Reminder::new(entry.id.clone(), trigger, Some(14));
        repository.save_reminder(reminder.clone()).await.unwrap();

        // Simulate scheduler query
        let reminders = repository.reminders().await.unwrap();
        assert_eq!(reminders.len(), 1);
        let target = &reminders[0];
        assert_eq!(target.status, "Scheduled");

        // Try to claim it
        let update_res = sqlx::query(
            "UPDATE reminders SET status = 'Triggered' WHERE id = ? AND status = 'Scheduled'",
        )
        .bind(target.id.value().to_string())
        .execute(&pool)
        .await
        .unwrap();

        assert_eq!(update_res.rows_affected(), 1);

        // Try to claim it again (should affect 0 rows)
        let update_res_again = sqlx::query(
            "UPDATE reminders SET status = 'Triggered' WHERE id = ? AND status = 'Scheduled'",
        )
        .bind(target.id.value().to_string())
        .execute(&pool)
        .await
        .unwrap();

        assert_eq!(update_res_again.rows_affected(), 0);
    }

    #[tokio::test]
    async fn test_search_entries() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        run_migrations(&pool).await.unwrap();
        let mut repository = SqliteChronologyRepository::new(pool);

        let category = Category::new("Garden").unwrap();
        repository.save_category(category.clone()).await.unwrap();

        let object = ChronicleObject::new(category.id, "Apple tree", None).unwrap();
        repository.save_object(object.clone()).await.unwrap();

        let entry1 = Entry::new(
            object.id,
            Utc::now() - chrono::Duration::days(2),
            "Sprayed fungicide",
            Some("Detailed info about spray".to_string()),
        )
        .unwrap();
        let entry2 = Entry::new(object.id, Utc::now(), "Watered the tree", None).unwrap();

        repository.save_entry(entry1.clone()).await.unwrap();
        repository.save_entry(entry2.clone()).await.unwrap();

        // 1. Search by text (should match entry1)
        let results = repository
            .search_entries(Some("spray".to_string()), None, None, None, None)
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Sprayed fungicide");

        // 2. Search by object_id
        let results = repository
            .search_entries(None, None, Some(object.id.value().to_string()), None, None)
            .await
            .unwrap();
        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_wrong_pin_verification() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        run_migrations(&pool).await.unwrap();

        let salt_bytes = crate::application::security::generate_salt();
        let salt_hex = crate::application::security::to_hex(&salt_bytes);
        let hash_hex = crate::application::security::hash_pin("1234", &salt_bytes);

        // Store salt and hash in metadata
        sqlx::query("INSERT OR REPLACE INTO app_metadata (key, value) VALUES ('pin_hash', ?)")
            .bind(&hash_hex)
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT OR REPLACE INTO app_metadata (key, value) VALUES ('pin_salt', ?)")
            .bind(&salt_hex)
            .execute(&pool)
            .await
            .unwrap();

        // 1. Verify correct PIN
        let entered_pin = "1234";
        let computed_hash = crate::application::security::hash_pin(entered_pin, &salt_bytes);
        assert_eq!(computed_hash, hash_hex);

        // 2. Verify wrong PIN
        let entered_wrong_pin = "9999";
        let computed_wrong_hash =
            crate::application::security::hash_pin(entered_wrong_pin, &salt_bytes);
        assert_ne!(computed_wrong_hash, hash_hex);
    }

    #[tokio::test]
    async fn test_full_backup_restore_cycle() {
        let temp_dir = std::env::temp_dir();
        let db_file = temp_dir.join("hroniki_test_db.sqlite");
        if db_file.exists() {
            let _ = std::fs::remove_file(&db_file);
        }

        let db_url = format!("sqlite://{}", db_file.to_string_lossy().replace('\\', "/"));
        let pool = create_pool(&db_url).await.unwrap();
        run_migrations(&pool).await.unwrap();
        let mut repository = SqliteChronologyRepository::new(pool.clone());

        let category = Category::new("Garden").unwrap();
        repository.save_category(category.clone()).await.unwrap();

        let object = ChronicleObject::new(category.id, "Apple tree", None).unwrap();
        repository.save_object(object.clone()).await.unwrap();

        let entry = Entry::new(object.id, Utc::now(), "Treatment", None).unwrap();
        repository.save_entry(entry.clone()).await.unwrap();

        let reminder = Reminder::new(entry.id.clone(), Utc::now(), Some(7));
        repository.save_reminder(reminder.clone()).await.unwrap();

        // Verify initial state
        assert_eq!(repository.categories().await.unwrap().len(), 1);
        assert_eq!(repository.objects().await.unwrap().len(), 1);
        assert_eq!(repository.entries().await.unwrap().len(), 1);
        assert_eq!(repository.reminders().await.unwrap().len(), 1);

        // Create backup database file path
        let backup_db_file = temp_dir.join("hroniki_backup_test.sqlite");
        if backup_db_file.exists() {
            let _ = std::fs::remove_file(&backup_db_file);
        }

        // Force WAL checkpoint to ensure all data is written to main file before VACUUM
        sqlx::query("PRAGMA wal_checkpoint(TRUNCATE);")
            .execute(&pool)
            .await
            .unwrap();

        // Export via VACUUM INTO
        sqlx::query(&format!(
            "VACUUM INTO '{}'",
            backup_db_file.to_string_lossy().replace('\\', "/")
        ))
        .execute(&pool)
        .await
        .unwrap();

        assert!(backup_db_file.exists());

        // Clear all tables
        sqlx::query("DELETE FROM reminders")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM entries")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM objects")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM categories")
            .execute(&pool)
            .await
            .unwrap();

        // Verify empty state
        assert_eq!(repository.categories().await.unwrap().len(), 0);
        assert_eq!(repository.objects().await.unwrap().len(), 0);
        assert_eq!(repository.entries().await.unwrap().len(), 0);
        assert_eq!(repository.reminders().await.unwrap().len(), 0);

        // Restore: close pool, copy file back, reopen pool
        pool.close().await;

        std::fs::copy(&backup_db_file, &db_file).unwrap();

        let restored_pool = create_pool(&db_url).await.unwrap();

        let restored_repo = SqliteChronologyRepository::new(restored_pool);
        assert_eq!(restored_repo.categories().await.unwrap().len(), 1);
        assert_eq!(restored_repo.objects().await.unwrap().len(), 1);
        assert_eq!(restored_repo.entries().await.unwrap().len(), 1);
        assert_eq!(restored_repo.reminders().await.unwrap().len(), 1);

        // Clean up
        let _ = std::fs::remove_file(&backup_db_file);
        let _ = std::fs::remove_file(&db_file);
    }
}
