use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    application::chronology::ChronologyService, platform::PlatformContext,
    reminder::ReminderService, search::SearchService, storage::SqliteChronologyRepository,
};

pub type AppService = ChronologyService<SqliteChronologyRepository>;

pub struct AppState {
    pub service: Arc<Mutex<AppService>>,
    pub event_bus: Arc<crate::events::EventBus>,
    pub search_service: Arc<SearchService>,
    pub reminder_service: Arc<ReminderService>,
    pub platform_context: Arc<PlatformContext>,
    pub session_manager: Arc<crate::platform::SessionManager>,
}
