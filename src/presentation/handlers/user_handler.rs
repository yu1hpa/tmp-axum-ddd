use axum::{
    Router,
    extract::{Path, State},
    response::{IntoResponse, Json},
    routing::get,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::models::user::{User, hash},
    usecase::user_usecase::UserService,
};

#[derive(Clone)]
pub struct AppState<T: UserService> {
    pub user_service: Arc<T>,
}

pub fn create_user_router<T: UserService + Send + Sync + 'static + Clone>(
    user_service: T,
) -> Router {
    let state = AppState {
        user_service: Arc::new(user_service),
    };

    Router::new()
        .route("/users", get(get_all::<T>).post(create::<T>))
        .route(
            "/users/{id}",
            get(get_by_id::<T>).put(update::<T>).delete(delete::<T>),
        )
        .with_state(state)
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    nickname: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct UpdateUserRequest {
    name: Option<String>,
    nickname: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

#[derive(Serialize)]
struct UserResponse {
    id: Uuid,
    name: String,
    nickname: String,
    email: String,
    hashed_password: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            nickname: user.nickname,
            email: user.email,
            hashed_password: user.hashed_password,
        }
    }
}

async fn get_all<T: UserService>(State(state): State<AppState<T>>) -> impl IntoResponse {
    match state.user_service.get_all().await {
        Ok(users) => Json(
            users
                .into_iter()
                .map(UserResponse::from)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch users").into_response(),
    }
}

async fn get_by_id<T: UserService>(
    State(state): State<AppState<T>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.user_service.get_by_id(id).await {
        Ok(Some(user)) => Json(UserResponse::from(user)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch user").into_response(),
    }
}

async fn create<T: UserService>(
    State(state): State<AppState<T>>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    match state
        .user_service
        .create(
            payload.name,
            payload.nickname,
            payload.email,
            hash(&payload.password),
        )
        .await
    {
        Ok(user) => (StatusCode::CREATED, Json(UserResponse::from(user))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user").into_response(),
    }
}

async fn update<T: UserService>(
    State(state): State<AppState<T>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    let hashed_password = payload.password.as_ref().map(|plain| hash(plain));

    match state
        .user_service
        .update(
            id,
            payload.name,
            payload.nickname,
            payload.email,
            hashed_password,
        )
        .await
    {
        Ok(user) => Json(UserResponse::from(user)).into_response(),
        Err(sqlx::Error::RowNotFound) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update user {}", e),
        )
            .into_response(),
    }
}

async fn delete<T: UserService>(
    State(state): State<AppState<T>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.user_service.delete(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(sqlx::Error::RowNotFound) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete user").into_response(),
    }
}
