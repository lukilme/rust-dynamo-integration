use axum::{Router, routing::post};
use aws_sdk_dynamodb::Client;

use crate::{handlers::users::create_user};

#[derive(Clone)]
pub struct AppState {
    pub dynamodb: Client,
    pub table: String,
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .with_state(state)
}
