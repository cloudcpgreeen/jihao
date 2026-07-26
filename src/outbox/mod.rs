use std::sync::Arc;
use std::time::Duration;

use event_contracts::{MemoryEventBus, UserCreated, UserProfileUpdated};

use crate::repository::UserRepo;

/// Background relay: polls the outbox table and publishes unpublished events.
///
/// This decouples "save domain state" from "publish event" — if the event bus
/// is down, events stay in the outbox and are retried next poll.
///
/// ponytail: polling every 1s, swap for LISTEN/NOTIFY when latency matters.
pub struct OutboxRelay<R: UserRepo> {
    repo: R,
    event_bus: Arc<MemoryEventBus>,
}

impl<R: UserRepo + Send + Sync + 'static> OutboxRelay<R> {
    pub fn new(repo: R, event_bus: Arc<MemoryEventBus>) -> Self {
        Self { repo, event_bus }
    }

    pub async fn run(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            self.poll().await;
        }
    }

    pub async fn poll(&self) {
        let events = match self.repo.fetch_unpublished_events() {
            Ok(events) => events,
            Err(e) => {
                eprintln!("[outbox-relay] fetch error: {:?}", e);
                return;
            }
        };

        for entry in events {
            match entry.event_type.as_str() {
                "UserCreated" => {
                    match serde_json::from_str::<UserCreated>(&entry.payload) {
                        Ok(event) => {
                            self.event_bus.publish(event);
                        }
                        Err(e) => {
                            eprintln!("[outbox-relay] deserialize UserCreated error: {:?}", e);
                            continue;
                        }
                    }
                }
                "UserProfileUpdated" => {
                    match serde_json::from_str::<UserProfileUpdated>(&entry.payload) {
                        Ok(event) => {
                            self.event_bus.publish(event);
                        }
                        Err(e) => {
                            eprintln!("[outbox-relay] deserialize UserProfileUpdated error: {:?}", e);
                            continue;
                        }
                    }
                }
                other => {
                    eprintln!("[outbox-relay] unknown event type: {}", other);
                    continue;
                }
            }
            if let Err(e) = self.repo.mark_published(&entry.id) {
                eprintln!("[outbox-relay] mark_published error: {:?}", e);
            }
        }
    }
}
