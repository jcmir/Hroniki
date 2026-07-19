use super::service::SearchService;
use crate::events::{DomainEvent, EventBus};
use sqlx::{Row, SqlitePool};
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct SearchSubscriber {
    event_bus: Arc<EventBus>,
    search_service: Arc<SearchService>,
    pool: SqlitePool,
}

impl SearchSubscriber {
    pub fn new(
        event_bus: Arc<EventBus>,
        search_service: Arc<SearchService>,
        pool: SqlitePool,
    ) -> Self {
        Self {
            event_bus,
            search_service,
            pool,
        }
    }

    pub fn start(self) {
        let mut rx = self.event_bus.subscribe();
        let service = self.search_service.clone();
        let pool = self.pool.clone();

        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(DomainEvent::EntryCreated {
                        entry_id,
                        object_id,
                    }) => {
                        // Load entry details from DB to get title, description
                        let row =
                            sqlx::query("SELECT title, description FROM entries WHERE id = ?")
                                .bind(&entry_id)
                                .fetch_optional(&pool)
                                .await;

                        if let Ok(Some(r)) = row {
                            let title: String = r.try_get("title").unwrap_or_default();
                            let description: Option<String> =
                                r.try_get("description").unwrap_or(None);

                            // Load tags for this entry
                            let tag_rows = sqlx::query(
                                "SELECT t.name FROM tags t JOIN entry_tags et ON t.id = et.tag_id WHERE et.entry_id = ?"
                            )
                            .bind(&entry_id)
                            .fetch_all(&pool)
                            .await
                            .unwrap_or_default();

                            let tags: Vec<String> = tag_rows
                                .iter()
                                .filter_map(|r| r.try_get::<String, _>("name").ok())
                                .collect();

                            let _ = service
                                .reindex_entry(
                                    &entry_id,
                                    &object_id,
                                    &title,
                                    description.as_deref(),
                                    &tags,
                                )
                                .await;
                        }
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        eprintln!("SearchSubscriber lagged, skipped {} events", n);
                    }
                    _ => {}
                }
            }
        });
    }
}
