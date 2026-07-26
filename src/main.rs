use std::sync::Arc;

use event_contracts::MemoryEventBus;
use user_svc::application;
use user_svc::outbox;
use user_svc::repository;

#[tokio::main]
async fn main() {
    let repo = repository::MemUserRepo::new();
    let event_bus = Arc::new(MemoryEventBus::new());

    let svc = Arc::new(application::UserService::new(repo.clone()));
    let app = user_svc::adapter::router(svc).merge(user_svc::adapter::region_router());

    // Background outbox relay: polls unpublished events, publishes to bus
    let relay = outbox::OutboxRelay::new(repo, event_bus);
    tokio::spawn(async move {
        relay.run().await;
    });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
