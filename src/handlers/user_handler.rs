use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{error::AppError, models::user::{CreateUser, UpdateUser}};
use crate::services::user_service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.create_user(payload).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.get_user(id).await?;
    Ok((StatusCode::OK, Json(user)))
}

pub async fn get_users(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let users = state.user_service.get_users().await?;
    Ok((StatusCode::OK, Json(users)))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.update_user(id, payload).await?;
    Ok((StatusCode::OK, Json(user)))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.user_service.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
