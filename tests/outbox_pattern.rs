/// Integration test: validates the Outbox pattern.
///
///   1. User is created → user + outbox entry saved atomically
///   2. Outbox relay publishes to event bus
///   3. Notification handler subscribed to event bus receives it
use std::sync::{Arc, Mutex};

use event_contracts::{MemoryEventBus, UserCreated};
use user_svc::application::UserService;
use user_svc::repository::{MemUserRepo, UserRepo};

#[test]
fn user_creation_inserts_outbox_entry_atomically() {
    let repo = MemUserRepo::new();
    let svc = UserService::new(repo.clone());

    // Before: outbox is empty
    let before = repo
        .fetch_unpublished_events()
        .expect("fetch ok");
    assert!(before.is_empty(), "outbox starts empty");

    // Create user
    let user = svc.create_user("pcode_bob", "bob").expect("create ok");
    assert_eq!(user.nickname, "bob");

    // After: outbox has one unpublished UserCreated event
    let after = repo
        .fetch_unpublished_events()
        .expect("fetch ok");
    assert_eq!(after.len(), 1, "one outbox entry created atomically");
    assert_eq!(after[0].event_type, "UserCreated");
    assert!(!after[0].published);

    // Deserialize and verify payload
    let event: UserCreated =
        serde_json::from_str(&after[0].payload).expect("valid UserCreated JSON");
    assert_eq!(event.user_id, user.id);
}

#[test]
fn relay_publishes_events_and_marks_published() {
    let repo = MemUserRepo::new();
    let event_bus = Arc::new(MemoryEventBus::new());
    let svc = UserService::new(repo.clone());

    // Capture events published to the bus
    let received: Arc<Mutex<Vec<UserCreated>>> = Arc::new(Mutex::new(Vec::new()));
    {
        let received = Arc::clone(&received);
        event_bus.subscribe::<UserCreated, _>(move |event: &UserCreated| {
            received.lock().unwrap().push(event.clone());
        });
    }

    // Create a user → outbox entry created
    let user = svc.create_user("pcode_carol", "carol").expect("create ok");

    // Before relay: event NOT yet on bus, outbox entry is unpublished
    assert!(received.lock().unwrap().is_empty());

    // Simulate relay poll: fetch unpublished → publish → mark published
    let unpublished = repo.fetch_unpublished_events().unwrap();
    assert_eq!(unpublished.len(), 1);

    for entry in &unpublished {
        let event: UserCreated = serde_json::from_str(&entry.payload).expect("deserialize ok");
        event_bus.publish(event);
        repo.mark_published(&entry.id).expect("mark ok");
    }

    // After relay: event on bus, outbox entry is published
    let received_events = received.lock().unwrap();
    assert_eq!(received_events.len(), 1);
    assert_eq!(received_events[0].user_id, user.id);

    // Outbox entry now marked published
    let remaining = repo.fetch_unpublished_events().unwrap();
    assert!(remaining.is_empty(), "all events published");

    // Relay repoll: no more unpublished events
    let repoll = repo.fetch_unpublished_events().unwrap();
    assert!(repoll.is_empty(), "nothing left to publish");
}

#[test]
fn create_user_failure_does_not_produce_outbox_entry() {
    let repo = MemUserRepo::new();
    let svc = UserService::new(repo.clone());

    // Empty nickname → validation error before repo write
    let result = svc.create_user("pcode_empty", "");
    assert!(result.is_err());

    // Outbox is still empty — no event emitted for failed creation
    let outbox = repo.fetch_unpublished_events().unwrap();
    assert!(outbox.is_empty());
}
