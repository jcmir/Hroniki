use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    application::chronology::ChronologyService,
    storage::SqliteChronologyRepository,
};


pub type AppService =
    ChronologyService<
        SqliteChronologyRepository
    >;


pub struct AppState {
    pub service: Arc<Mutex<AppService>>,
    pub event_bus: Arc<crate::events::EventBus>,
}
