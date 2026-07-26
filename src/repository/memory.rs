use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use chrono::Utc;
use uuid::Uuid;

use crate::domain::{Error, OutboxEntry, Profile, Status, User, UserAddress};
use super::UserRepo;

/// Single Mutex wrapping both users and outbox — same lock = same "transaction".
///
/// ponytail: global Mutex, swap for pg with real BEGIN/COMMIT when DB lands.
struct State {
    users: HashMap<String, User>,
    outbox: Vec<OutboxEntry>,
    addresses: Vec<UserAddress>,
}

#[derive(Clone)]
pub struct MemUserRepo {
    state: Arc<Mutex<State>>,
}

impl MemUserRepo {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        let now = Utc::now();
        users.insert(
            "usr_seed01".into(),
            User {
                id: "usr_seed01".into(),
                principal_code: "pcode_seed01".into(),
                nickname: "alice".into(),
                avatar: "https://example.com/alice.png".into(),
                email: "alice@example.com".into(),
                verified_name: None,
                status: Status::Active,
                created_at: now,
                updated_at: now,
            },
        );
        Self {
            state: Arc::new(Mutex::new(State {
                users,
                outbox: Vec::new(),
                addresses: Vec::new(),
            })),
        }
    }
}

impl UserRepo for MemUserRepo {
    fn create_user(&self, principal_code: &str, nickname: &str) -> Result<User, Error> {
        if nickname.is_empty() {
            return Err(Error::InvalidInput("nickname must not be empty".into()));
        }
        let now = Utc::now();
        let user = User {
            id: format!("usr_{}", Uuid::new_v4()),
            principal_code: principal_code.into(),
            nickname: nickname.into(),
            avatar: String::new(),
            email: String::new(),
            verified_name: None,
            status: Status::Active,
            created_at: now,
            updated_at: now,
        };

        let mut state = self.state.lock().unwrap();
        state.users.insert(user.id.clone(), user.clone());

        // Insert outbox event atomically
        let payload = serde_json::json!({
            "user_id": user.id,
            "principal_code": user.principal_code,
            "email": user.email,
            "created_at": now.to_rfc3339(),
        })
        .to_string();
        state.outbox.push(OutboxEntry {
            id: Uuid::new_v4().to_string(),
            event_type: "UserCreated".into(),
            payload,
            published: false,
        });

        Ok(user)
    }

    fn get_user(&self, id: &str) -> Result<Option<User>, Error> {
        let state = self.state.lock().unwrap();
        Ok(state.users.get(id).cloned())
    }

    fn get_profile(&self, id: &str) -> Result<Option<Profile>, Error> {
        let state = self.state.lock().unwrap();
        Ok(state.users.get(id).map(Profile::from))
    }

    fn update_profile(&self, id: &str, nickname: &str, avatar: &str) -> Result<Profile, Error> {
        let mut state = self.state.lock().unwrap();
        let user = state.users.get_mut(id).ok_or(Error::NotFound)?;

        if nickname.is_empty() {
            return Err(Error::InvalidInput("nickname must not be empty".into()));
        }

        user.nickname = nickname.into();
        user.avatar = avatar.into();
        user.updated_at = Utc::now();

        // ponytail: capture values before pushing to outbox to avoid double borrow
        let user_id = user.id.clone();
        let principal_code = user.principal_code.clone();
        let nick = user.nickname.clone();
        let av = user.avatar.clone();
        let updated = user.updated_at;
        let profile = Profile::from(&*user);

        let payload = serde_json::json!({
            "user_id": user_id,
            "principal_code": principal_code,
            "changes": {
                "nickname": nick,
                "avatar": av,
            },
            "updated_at": updated.to_rfc3339(),
        })
        .to_string();
        state.outbox.push(OutboxEntry {
            id: Uuid::new_v4().to_string(),
            event_type: "UserProfileUpdated".into(),
            payload,
            published: false,
        });

        Ok(profile)
    }

    fn update_status(&self, id: &str, new_status: Status) -> Result<User, Error> {
        let mut state = self.state.lock().unwrap();
        let user = state.users.get_mut(id).ok_or(Error::NotFound)?;
        user.status = new_status;
        user.updated_at = Utc::now();
        Ok(user.clone())
    }

    fn mark_verified(&self, id: &str, verified_name: &str) -> Result<(), Error> {
        let mut state = self.state.lock().unwrap();
        let user = state.users.get_mut(id).ok_or(Error::NotFound)?;
        user.verified_name = Some(verified_name.into());
        user.updated_at = Utc::now();
        Ok(())
    }

    fn save_address(
        &self,
        user_id: &str,
        province: &str,
        city: &str,
        district: &str,
        detail: &str,
        is_default: bool,
    ) -> Result<UserAddress, Error> {
        let mut state = self.state.lock().unwrap();

        // Ensure user exists
        if !state.users.contains_key(user_id) {
            return Err(Error::NotFound);
        }

        // Unset other defaults if this one is default
        if is_default {
            for a in state.addresses.iter_mut() {
                if a.user_id == user_id {
                    a.is_default = false;
                }
            }
        }

        let addr = UserAddress {
            id: format!("addr_{}", Uuid::new_v4()),
            user_id: user_id.into(),
            province: province.into(),
            city: city.into(),
            district: district.into(),
            detail: detail.into(),
            is_default,
            created_at: Utc::now(),
        };
        state.addresses.push(addr.clone());
        Ok(addr)
    }

    fn get_addresses(&self, user_id: &str) -> Result<Vec<UserAddress>, Error> {
        let state = self.state.lock().unwrap();
        Ok(state
            .addresses
            .iter()
            .filter(|a| a.user_id == user_id)
            .cloned()
            .collect())
    }

    fn set_default_address(&self, user_id: &str, address_id: &str) -> Result<(), Error> {
        let mut state = self.state.lock().unwrap();
        let mut found = false;
        for a in state.addresses.iter_mut() {
            if a.user_id == user_id {
                a.is_default = a.id == address_id;
                if a.id == address_id {
                    found = true;
                }
            }
        }
        if found {
            Ok(())
        } else {
            Err(Error::NotFound)
        }
    }

    fn delete_address(&self, user_id: &str, address_id: &str) -> Result<(), Error> {
        let mut state = self.state.lock().unwrap();
        let pos = state
            .addresses
            .iter()
            .position(|a| a.id == address_id && a.user_id == user_id);
        match pos {
            Some(i) => {
                state.addresses.remove(i);
                Ok(())
            }
            None => Err(Error::NotFound),
        }
    }

    fn fetch_unpublished_events(&self) -> Result<Vec<OutboxEntry>, Error> {
        let state = self.state.lock().unwrap();
        Ok(state
            .outbox
            .iter()
            .filter(|e| !e.published)
            .cloned()
            .collect())
    }

    fn mark_published(&self, id: &str) -> Result<(), Error> {
        let mut state = self.state.lock().unwrap();
        if let Some(entry) = state.outbox.iter_mut().find(|e| e.id == id) {
            entry.published = true;
        }
        Ok(())
    }
}
