use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Administrative division — loaded once from data/regions.json at compile time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub code: String,
    pub name: String,
    pub level: String, // "province" | "city" | "district"
    #[serde(default)]
    pub parent_code: Option<String>,
}

/// User's saved address.
#[derive(Debug, Clone)]
pub struct UserAddress {
    pub id: String,
    pub user_id: String,
    pub province: String,
    pub city: String,
    pub district: String,
    pub detail: String,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
}
