use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    ValidationError(String),
    DatabaseError(String),
    NotFound(String),
    Unauthorized(String),
    InternalServerError(String),
}

impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        let error_messages = err
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                format!(
                    "{}: {}",
                    field,
                    errors[0]
                        .message
                        .as_ref()
                        .map_or("Unknown error", |v| v)
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        
        AppError::ValidationError(error_messages)
    }
}

impl From<aws_sdk_dynamodb::Error> for AppError {
    fn from(err: aws_sdk_dynamodb::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AppError::InternalServerError(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

// Macro para facilitar a criação de erros
#[macro_export]
macro_rules! bail {
    ($variant:ident, $msg:literal $(,)?) => {
        return Err(crate::error::AppError::$variant($msg.to_string()))
    };
    ($variant:ident, $msg:expr $(,)?) => {
        return Err(crate::error::AppError::$variant($msg))
    };
    ($variant:ident, $fmt:expr, $($arg:tt)*) => {
        return Err(crate::error::AppError::$variant(format!($fmt, $($arg)*)))
    };
}