use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::application::UserService;
use crate::domain;
use crate::repository::MemUserRepo;

// -- DTOs --

#[derive(Serialize)]
struct UserResponse {
    id: String,
    nickname: String,
    avatar: String,
    status: String,
    email: String,
    verified_name: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize)]
struct ProfileResponse {
    id: String,
    nickname: String,
    avatar: String,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    principal_code: String,
    nickname: String,
}

#[derive(Deserialize)]
struct UpdateProfileRequest {
    nickname: String,
    #[serde(default)]
    avatar: String,
}

#[derive(Deserialize)]
struct SaveAddressRequest {
    province: String,
    city: String,
    district: String,
    detail: String,
    #[serde(default)]
    is_default: bool,
}

#[derive(Serialize)]
struct AddressResponse {
    id: String,
    user_id: String,
    province: String,
    city: String,
    district: String,
    detail: String,
    is_default: bool,
    created_at: String,
}

// -- error mapping --

fn map_error(e: domain::Error) -> (StatusCode, String) {
    match e {
        domain::Error::NotFound => (StatusCode::NOT_FOUND, "not found".into()),
        domain::Error::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
        domain::Error::AlreadyExists => (StatusCode::CONFLICT, "already exists".into()),
    }
}

// -- handlers --

type AppState = Arc<UserService<MemUserRepo>>;

async fn get_user(State(svc): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match svc.get_user(&id) {
        Ok(Some(u)) => Json(&UserResponse {
            id: u.id,
            nickname: u.nickname,
            avatar: u.avatar,
            status: format!("{:?}", u.status).to_lowercase(),
            email: u.email,
            verified_name: u.verified_name,
            created_at: u.created_at.to_rfc3339(),
            updated_at: u.updated_at.to_rfc3339(),
        })
        .into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "not found").into_response(),
        Err(e) => {
            let (status, msg) = map_error(e);
            (status, msg).into_response()
        }
    }
}

async fn get_profile(State(svc): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match svc.get_profile(&id) {
        Ok(Some(p)) => Json(&ProfileResponse {
            id: p.id,
            nickname: p.nickname,
            avatar: p.avatar,
        })
        .into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "not found").into_response(),
        Err(e) => {
            let (status, msg) = map_error(e);
            (status, msg).into_response()
        }
    }
}

async fn create_user(
    State(svc): State<AppState>,
    Json(body): Json<CreateUserRequest>,
) -> impl IntoResponse {
    match svc.create_user(&body.principal_code, &body.nickname) {
        Ok(u) => (
            StatusCode::CREATED,
            Json(&UserResponse {
                id: u.id,
                nickname: u.nickname,
                avatar: u.avatar,
                status: format!("{:?}", u.status).to_lowercase(),
                email: u.email,
                verified_name: u.verified_name,
                created_at: u.created_at.to_rfc3339(),
                updated_at: u.updated_at.to_rfc3339(),
            }),
        )
            .into_response(),
        Err(e) => {
            let (status, msg) = map_error(e);
            (status, msg).into_response()
        }
    }
}

async fn update_profile(
    State(svc): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<UpdateProfileRequest>,
) -> impl IntoResponse {
    match svc.update_profile(&id, &body.nickname, &body.avatar) {
        Ok(p) => Json(&ProfileResponse {
            id: p.id,
            nickname: p.nickname,
            avatar: p.avatar,
        })
        .into_response(),
        Err(e) => {
            let (status, msg) = map_error(e);
            (status, msg).into_response()
        }
    }
}

// -- address handlers --

async fn save_address(
    State(svc): State<AppState>,
    Path(user_id): Path<String>,
    Json(body): Json<SaveAddressRequest>,
) -> impl IntoResponse {
    match svc.save_address(
        &user_id,
        &body.province,
        &body.city,
        &body.district,
        &body.detail,
        body.is_default,
    ) {
        Ok(a) => (
            StatusCode::CREATED,
            Json(&AddressResponse {
                id: a.id,
                user_id: a.user_id,
                province: a.province,
                city: a.city,
                district: a.district,
                detail: a.detail,
                is_default: a.is_default,
                created_at: a.created_at.to_rfc3339(),
            }),
        )
            .into_response(),
        Err(e) => {
            let (status, msg) = map_error(e);
            (status, msg).into_response()
        }
    }
}

async fn list_addresses(
    State(svc): State<AppState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    match svc.get_addresses(&user_id) {
        Ok(addresses) => Json(
            addresses
                .iter()
                .map(|a| AddressResponse {
                    id: a.id.clone(),
                    user_id: a.user_id.clone(),
                    province: a.province.clone(),
                    city: a.city.clone(),
                    district: a.district.clone(),
                    detail: a.detail.clone(),
                    is_default: a.is_default,
                    created_at: a.created_at.to_rfc3339(),
                })
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(e) => {
            let (status, msg) = map_error(e);
            (status, msg).into_response()
        }
    }
}

async fn set_default_address(
    State(svc): State<AppState>,
    Path((user_id, address_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match svc.set_default_address(&user_id, &address_id) {
        Ok(()) => StatusCode::OK.into_response(),
        Err(e) => {
            let (status, msg) = map_error(e);
            (status, msg).into_response()
        }
    }
}

async fn delete_address(
    State(svc): State<AppState>,
    Path((user_id, address_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match svc.delete_address(&user_id, &address_id) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            let (status, msg) = map_error(e);
            (status, msg).into_response()
        }
    }
}

pub fn router(svc: Arc<UserService<MemUserRepo>>) -> Router {
    Router::new()
        .route("/api/users", post(create_user))
        .route("/api/users/{id}", get(get_user))
        .route("/api/users/{id}/profile", get(get_profile).put(update_profile))
        .route("/api/users/{id}/addresses", get(list_addresses).post(save_address))
        .route(
            "/api/users/{id}/addresses/{addr_id}",
            delete(delete_address),
        )
        .route(
            "/api/users/{id}/addresses/{addr_id}/default",
            post(set_default_address),
        )
        .with_state(svc)
}
