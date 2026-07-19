use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    application::chronology::ChronologyService, search::SearchService,
    storage::SqliteChronologyRepository,
};

pub type AppService = ChronologyService<SqliteChronologyRepository>;

pub struct AppState {
    pub service: Arc<Mutex<AppService>>,
    pub event_bus: Arc<crate::events::EventBus>,
    pub search_service: Arc<SearchService>,
}
