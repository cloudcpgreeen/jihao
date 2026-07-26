/// Component trait for cloud:user/user WIT interface.
///
/// 1:1 Rust translation of the WIT contract.
use crate::application::UserService;
use crate::domain::{Error, Profile, Status, User, UserAddress};
use crate::repository::MemUserRepo;

pub trait UserApi {
    fn create_user(&self, principal_code: &str, nickname: &str) -> Result<User, Error>;
    fn get_user(&self, id: &str) -> Result<Option<User>, Error>;
    fn get_profile(&self, id: &str) -> Result<Option<Profile>, Error>;
    fn update_profile(&self, id: &str, nickname: &str, avatar: &str) -> Result<Profile, Error>;
    fn update_status(&self, id: &str, new_status: Status) -> Result<User, Error>;
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
    fn mark_verified(&self, id: &str, verified_name: &str) -> Result<(), Error>;
}

/// Memory implementation — delegates to UserService<MemUserRepo>.
impl UserApi for UserService<MemUserRepo> {
    fn create_user(&self, principal_code: &str, nickname: &str) -> Result<User, Error> {
        self.create_user(principal_code, nickname)
    }
    fn get_user(&self, id: &str) -> Result<Option<User>, Error> {
        self.get_user(id)
    }
    fn get_profile(&self, id: &str) -> Result<Option<Profile>, Error> {
        self.get_profile(id)
    }
    fn update_profile(&self, id: &str, nickname: &str, avatar: &str) -> Result<Profile, Error> {
        self.update_profile(id, nickname, avatar)
    }
    fn update_status(&self, id: &str, new_status: Status) -> Result<User, Error> {
        self.update_status(id, new_status)
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
        self.save_address(user_id, province, city, district, detail, is_default)
    }
    fn get_addresses(&self, user_id: &str) -> Result<Vec<UserAddress>, Error> {
        self.get_addresses(user_id)
    }
    fn set_default_address(&self, user_id: &str, address_id: &str) -> Result<(), Error> {
        self.set_default_address(user_id, address_id)
    }
    fn delete_address(&self, user_id: &str, address_id: &str) -> Result<(), Error> {
        self.delete_address(user_id, address_id)
    }
    fn mark_verified(&self, id: &str, verified_name: &str) -> Result<(), Error> {
        self.mark_verified(id, verified_name)
    }
}
