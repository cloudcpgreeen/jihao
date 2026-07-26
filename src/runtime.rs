/// Runtime adapter — proves User Kernel satisfies cloudos-runtime-core primitives.
///
/// User is a Projection Object — Principal remains the source of truth.
/// No changes to existing orchestration. Just trait impls alongside.
use cloudos_runtime_core::lifecycle::{Lifecycle, LifecycleAware};
use cloudos_runtime_core::named::Named;
use cloudos_runtime_core::object::Object;
use cloudos_runtime_core::transition::Transition;
use event_contracts::PrincipalCreated;

use crate::domain::{Status, User};

impl Object for User {
    fn id(&self) -> &str {
        &self.id
    }
}

impl LifecycleAware for User {
    fn lifecycle(&self) -> Lifecycle {
        Lifecycle::Projected
    }
}

impl Named for User {
    fn name(&self) -> &str {
        "identity.user"
    }
}

/// Pure transition: PrincipalCreated → User projection.
/// Takes user_id and nickname as input — ID generation belongs to User Kernel.
/// Deterministic: parses event.created_at, fails on bad input. No Utc::now().
pub struct PrincipalCreatedToUserProjection {
    pub user_id: String,
    pub nickname: String,
}

impl Transition for PrincipalCreatedToUserProjection {
    type Event = PrincipalCreated;
    type Output = Result<User, String>;

    fn apply(&self, event: PrincipalCreated) -> Result<User, String> {
        let created_at = chrono::DateTime::parse_from_rfc3339(&event.created_at)
            .map_err(|e| format!("invalid created_at: {}", e))?;
        let ts: chrono::DateTime<chrono::Utc> = created_at.into();
        Ok(User {
            id: self.user_id.clone(),
            principal_code: event.principal_code,
            nickname: self.nickname.clone(),
            avatar: String::new(),
            email: String::new(),
            verified_name: None,
            status: Status::Active,
            created_at: ts,
            updated_at: ts,
        })
    }
}

#[cfg(test)]
mod tests {
    use cloudos_runtime_core::lifecycle::Lifecycle;
    use cloudos_runtime_core::object::Object;
    use cloudos_runtime_core::transition::Transition;

    use super::*;

    fn fixed_time() -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::parse_from_rfc3339("2026-07-25T10:00:00Z")
            .unwrap()
            .into()
    }

    #[test]
    fn user_satisfies_object() {
        let ts = fixed_time();
        let u = User {
            id: "usr_test".into(),
            principal_code: "prn_test".into(),
            nickname: "tester".into(),
            avatar: String::new(),
            email: String::new(),
            verified_name: None,
            status: Status::Active,
            created_at: ts,
            updated_at: ts,
        };
        assert_eq!(u.id(), "usr_test");
        assert_eq!(u.lifecycle(), Lifecycle::Projected);
    }

    #[test]
    fn principal_created_to_user_transition_is_deterministic() {
        let t = PrincipalCreatedToUserProjection {
            user_id: "usr_test".into(),
            nickname: "tester".into(),
        };
        let u = t
            .apply(PrincipalCreated {
                principal_id: "prn_test".into(),
                principal_code: "prn_test".into(),
                created_at: "2026-07-25T10:00:00Z".into(),
            })
            .expect("valid event");
        assert_eq!(u.id(), "usr_test");
        assert_eq!(u.principal_code, "prn_test");
        assert_eq!(u.nickname, "tester");
        assert_eq!(u.lifecycle(), Lifecycle::Projected);
        assert_eq!(u.created_at, u.updated_at);
    }

    #[test]
    fn bad_event_fails_deterministically() {
        let t = PrincipalCreatedToUserProjection {
            user_id: "usr_test".into(),
            nickname: "tester".into(),
        };
        let result = t.apply(PrincipalCreated {
            principal_id: "prn_test".into(),
            principal_code: "prn_test".into(),
            created_at: "bad-timestamp".into(),
        });
        assert!(result.is_err());
    }
}
