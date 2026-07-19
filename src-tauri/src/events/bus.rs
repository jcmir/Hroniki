use tokio::sync::broadcast;
use super::event::DomainEvent;

pub struct EventBus {
    sender: broadcast::Sender<DomainEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    pub fn publish(&self, event: DomainEvent) {
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<DomainEvent> {
        self.sender.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_bus_pub_sub() {
        let bus = EventBus::new();
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();

        let event = DomainEvent::UserRegistered {
            user_id: "user-123".to_string(),
            email: Some("test@example.com".to_string()),
        };

        bus.publish(event.clone());

        let received1 = rx1.recv().await.unwrap();
        let received2 = rx2.recv().await.unwrap();

        if let DomainEvent::UserRegistered { user_id, email } = received1 {
            assert_eq!(user_id, "user-123");
            assert_eq!(email, Some("test@example.com".to_string()));
        } else {
            panic!("Expected UserRegistered event");
        }

        if let DomainEvent::UserRegistered { user_id, email } = received2 {
            assert_eq!(user_id, "user-123");
            assert_eq!(email, Some("test@example.com".to_string()));
        } else {
            panic!("Expected UserRegistered event");
        }
    }
}
