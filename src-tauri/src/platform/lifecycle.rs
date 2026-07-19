use crate::events::{DomainEvent, EventBus};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifecycleEvent {
    AppStarted,
    AppBackgrounded,
    AppSuspended,
    AppResumed,
    AppClosed,
    Unknown(String),
}

pub struct LifecycleTranslator {
    event_bus: Arc<EventBus>,
}

impl LifecycleTranslator {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }

    pub fn translate(&self, event: LifecycleEvent) {
        let domain_event = match event {
            LifecycleEvent::AppStarted => Some(DomainEvent::ApplicationStarted),
            LifecycleEvent::AppBackgrounded => Some(DomainEvent::ApplicationSuspended),
            LifecycleEvent::AppSuspended => Some(DomainEvent::ApplicationSuspended),
            LifecycleEvent::AppResumed => Some(DomainEvent::ApplicationResumed),
            LifecycleEvent::AppClosed => Some(DomainEvent::ApplicationClosed),
            LifecycleEvent::Unknown(name) => {
                tracing::warn!(event_name = ?name, "Received unknown OS lifecycle event callback");
                None
            }
        };

        if let Some(de) = domain_event {
            self.event_bus.publish(de);
        }
    }
}
