use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::domain::Region;

// -- store --

/// Static region data loaded at compile time from data/regions.json.
pub struct RegionStore {
    regions: Vec<Region>,
    by_parent: HashMap<String, Vec<usize>>, // parent_code → indices into regions
}

impl RegionStore {
    pub fn new() -> Self {
        let json = include_str!("../../data/regions.json");
        let regions: Vec<Region> = serde_json::from_str(json).expect("invalid regions.json");
        let mut by_parent: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, r) in regions.iter().enumerate() {
            if let Some(ref parent) = r.parent_code {
                by_parent.entry(parent.clone()).or_default().push(i);
            }
        }
        Self {
            regions,
            by_parent,
        }
    }

    pub fn provinces(&self) -> Vec<&Region> {
        self.regions
            .iter()
            .filter(|r| r.level == "province")
            .collect()
    }

    pub fn children(&self, parent_code: &str) -> Vec<&Region> {
        self.by_parent
            .get(parent_code)
            .map(|indices| indices.iter().map(|&i| &self.regions[i]).collect())
            .unwrap_or_default()
    }
}

// -- DTOs --

#[derive(Serialize)]
struct RegionItem {
    code: String,
    name: String,
}

// -- handlers --

#[derive(Deserialize)]
struct ChildrenQuery {
    province_code: Option<String>,
    city_code: Option<String>,
}

async fn get_provinces(State(store): State<Arc<RegionStore>>) -> Json<Vec<RegionItem>> {
    Json(
        store
            .provinces()
            .into_iter()
            .map(|r| RegionItem {
                code: r.code.clone(),
                name: r.name.clone(),
            })
            .collect(),
    )
}

async fn get_children(
    State(store): State<Arc<RegionStore>>,
    Query(q): Query<ChildrenQuery>,
) -> Json<Vec<RegionItem>> {
    let parent_code = q
        .city_code
        .or(q.province_code)
        .unwrap_or_default();
    Json(
        store
            .children(&parent_code)
            .into_iter()
            .map(|r| RegionItem {
                code: r.code.clone(),
                name: r.name.clone(),
            })
            .collect(),
    )
}

pub fn region_router() -> Router {
    let store = Arc::new(RegionStore::new());
    Router::new()
        .route("/api/regions/provinces", get(get_provinces))
        .route("/api/regions/children", get(get_children))
        .with_state(store)
}
