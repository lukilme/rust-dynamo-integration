use axum::{
    extract::State,
    Json,
};
use tracing::info;
use validator::Validate;
use aws_sdk_dynamodb::types::AttributeValue;

use crate::error::AppError; 
use crate::models::user::{CreateUserRequest, UserResponse, User};
use crate::AppState;

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    payload.validate()?;

    let user = User::from(payload);
    
    info!("Creating user: {:?}", user);
    
    state
        .dynamodb
        .put_item()
        .table_name(&state.table)
        .item("id", AttributeValue::S(user.id.clone()))
        .item("name", AttributeValue::S(user.name.clone()))
        .item("email", AttributeValue::S(user.email.clone()))
        .item("password", AttributeValue::S(user.password.clone()))
        .item("created_at", AttributeValue::S(user.created_at.to_rfc3339()))
        .send()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let response = UserResponse::from(user);
    
    Ok(Json(response))
}