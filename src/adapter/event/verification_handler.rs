use std::sync::Arc;

use event_contracts::{MemoryEventBus, VerificationApproved};

use crate::application::UserService;
use crate::repository::MemUserRepo;

/// Subscribe to VerificationApproved events.
/// Sets the user's verified_name — does NOT overwrite nickname.
pub fn subscribe_verification_approved(
    event_bus: &MemoryEventBus,
    svc: Arc<UserService<MemUserRepo>>,
) {
    event_bus.subscribe::<VerificationApproved, _>(move |event: &VerificationApproved| {
        println!(
            "[user-svc] VerificationApproved user_id={} name={}",
            event.user_id, event.name
        );

        match svc.mark_verified(&event.user_id, &event.name) {
            Ok(()) => println!(
                "[user-svc] user {} verified as '{}'",
                event.user_id, event.name
            ),
            Err(e) => eprintln!("[user-svc] mark_verified failed: {:?}", e),
        }
    });
}
