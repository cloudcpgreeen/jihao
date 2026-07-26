use crate::domain::{Error, OutboxEntry, Profile, Status, User, UserAddress};

pub trait UserRepo {
    fn create_user(&self, principal_code: &str, nickname: &str) -> Result<User, Error>;
    fn get_user(&self, id: &str) -> Result<Option<User>, Error>;
    fn get_profile(&self, id: &str) -> Result<Option<Profile>, Error>;
    fn update_profile(&self, id: &str, nickname: &str, avatar: &str) -> Result<Profile, Error>;
    fn update_status(&self, id: &str, new_status: Status) -> Result<User, Error>;
    fn mark_verified(&self, id: &str, verified_name: &str) -> Result<(), Error>;

    /// Address management
    fn save_address(
        &self,
        user_id: &str,
        province: &str,
        city: &str,
        district: &str,
        detail: &str,
        is_default: bool,
    ) -> Result<UserAddress, Error>;
    fn get_addresses(&self, user_id: &str) -> Result<Vec<UserAddress>, Error>;
    fn set_default_address(&self, user_id: &str, address_id: &str) -> Result<(), Error>;
    fn delete_address(&self, user_id: &str, address_id: &str) -> Result<(), Error>;

    /// Outbox — transactional with domain writes
    fn fetch_unpublished_events(&self) -> Result<Vec<OutboxEntry>, Error>;
    fn mark_published(&self, id: &str) -> Result<(), Error>;
}

mod memory;
mod pg;
pub use memory::MemUserRepo;
pub use pg::PgUserRepo;
