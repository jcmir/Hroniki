use crate::events::{DomainEvent, EventBus};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifecycleEvent {
    AppStarted,
    AppBackgrounded,
    AppSuspended,
    AppResumed,
    AppClosed,
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
            LifecycleEvent::AppStarted => DomainEvent::ApplicationStarted,
            LifecycleEvent::AppBackgrounded => DomainEvent::ApplicationSuspended,
            LifecycleEvent::AppSuspended => DomainEvent::ApplicationSuspended,
            LifecycleEvent::AppResumed => DomainEvent::ApplicationResumed,
            LifecycleEvent::AppClosed => DomainEvent::ApplicationClosed,
        };

        self.event_bus.publish(domain_event);
    }
}
