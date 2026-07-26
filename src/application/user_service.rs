use crate::domain::{Error, Profile, Status, User, UserAddress};
use crate::repository::UserRepo;

/// Pure business logic. No transport, no event bus — just the WIT contract in Rust.
/// Events are published via the outbox relay, not from here.
pub struct UserService<R: UserRepo> {
    repo: R,
}

impl<R: UserRepo> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn create_user(&self, principal_code: &str, nickname: &str) -> Result<User, Error> {
        if nickname.is_empty() {
            return Err(Error::InvalidInput("nickname must not be empty".into()));
        }
        self.repo.create_user(principal_code, nickname)
    }

    pub fn get_user(&self, id: &str) -> Result<Option<User>, Error> {
        self.repo.get_user(id)
    }

    pub fn get_profile(&self, id: &str) -> Result<Option<Profile>, Error> {
        self.repo.get_profile(id)
    }

    pub fn update_profile(
        &self,
        id: &str,
        nickname: &str,
        avatar: &str,
    ) -> Result<Profile, Error> {
        if nickname.is_empty() {
            return Err(Error::InvalidInput("nickname must not be empty".into()));
        }
        self.repo.update_profile(id, nickname, avatar)
    }

    pub fn save_address(
        &self,
        user_id: &str,
        province: &str,
        city: &str,
        district: &str,
        detail: &str,
        is_default: bool,
    ) -> Result<UserAddress, Error> {
        self.repo
            .save_address(user_id, province, city, district, detail, is_default)
    }

    pub fn get_addresses(&self, user_id: &str) -> Result<Vec<UserAddress>, Error> {
        self.repo.get_addresses(user_id)
    }

    pub fn set_default_address(&self, user_id: &str, address_id: &str) -> Result<(), Error> {
        self.repo.set_default_address(user_id, address_id)
    }

    pub fn delete_address(&self, user_id: &str, address_id: &str) -> Result<(), Error> {
        self.repo.delete_address(user_id, address_id)
    }

    pub fn update_status(&self, id: &str, new_status: Status) -> Result<User, Error> {
        self.repo.update_status(id, new_status)
    }

    pub fn mark_verified(&self, id: &str, verified_name: &str) -> Result<(), Error> {
        self.repo.mark_verified(id, verified_name)
    }
}
