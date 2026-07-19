use super::service::ReminderService;
use crate::events::EventBus;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct ReminderSubscriber {
    event_bus: Arc<EventBus>,
    reminder_service: Arc<ReminderService>,
}

impl ReminderSubscriber {
    pub fn new(event_bus: Arc<EventBus>, reminder_service: Arc<ReminderService>) -> Self {
        Self {
            event_bus,
            reminder_service,
        }
    }

    pub fn start(self) {
        let mut rx = self.event_bus.subscribe();
        let _service = self.reminder_service.clone();

        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Err(broadcast::error::RecvError::Closed) => break,
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        eprintln!("[ReminderSubscriber] Lagged behind {} events", n);
                    }
                    _ => {}
                }
            }
        });
    }
}
