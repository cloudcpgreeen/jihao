use chrono::{DateTime, Utc};

/// Mirrors `user` record in wit/user.wit
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub principal_code: String,
    pub nickname: String,
    pub avatar: String,
    pub email: String,
    pub verified_name: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Mirrors `profile` record in wit/user.wit
#[derive(Debug, Clone)]
pub struct Profile {
    pub id: String,
    pub nickname: String,
    pub avatar: String,
}

impl From<&User> for Profile {
    fn from(u: &User) -> Self {
        Self {
            id: u.id.clone(),
            nickname: u.nickname.clone(),
            avatar: u.avatar.clone(),
        }
    }
}

/// Mirrors `status` variant in wit/user.wit
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Inactive,
    Suspended,
}

/// Mirrors `error` variant in wit/user.wit
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NotFound,
    InvalidInput(String),
    AlreadyExists,
}

/// An event waiting to be published. Stored atomically with the domain change.
#[derive(Debug, Clone)]
pub struct OutboxEntry {
    pub id: String,
    pub event_type: String,
    pub payload: String,
    pub published: bool,
}
