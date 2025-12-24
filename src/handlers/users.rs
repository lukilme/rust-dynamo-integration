use axum::{Json, extract::State};
use aws_sdk_dynamodb::types::AttributeValue;
use uuid::Uuid;

use crate::{AppState, models::user::User};

pub async fn create_user(
    State(state): State<AppState>,
    Json(mut user): Json<User>,
) -> Json<User> {
    user.id = Uuid::new_v4().to_string();

    state
        .dynamodb
        .put_item()
        .table_name(&state.table)
        .item("id", AttributeValue::S(user.id.clone()))
        .item("name", AttributeValue::S(user.name.clone()))
        .send()
        .await
        .expect("erro ao inserir");

    Json(user)
}
