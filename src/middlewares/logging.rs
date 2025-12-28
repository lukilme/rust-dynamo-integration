use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use std::time::Instant;
use tracing::info;

pub async fn request_logger(req: Request<Body>, next: Next) -> impl IntoResponse {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();
    
    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status();

    match status {
        s if s.is_success() => {
            info!(
                method = %method,
                uri = %uri,
                status = %s.as_u16(),
                duration_ms = duration.as_millis(),
                "Request completed"
            );
        }
        s if s.is_client_error() => {
            tracing::warn!(
                method = %method,
                uri = %uri,
                status = %s.as_u16(),
                duration_ms = duration.as_millis(),
                "Client error"
            );
        }
        s if s.is_server_error() => {
            tracing::error!(
                method = %method,
                uri = %uri,
                status = %s.as_u16(),
                duration_ms = duration.as_millis(),
                "Server error"
            );
        }
        _ => {
            tracing::debug!(
                method = %method,
                uri = %uri,
                status = %status.as_u16(),
                duration_ms = duration.as_millis(),
                "Request"
            );
        }
    }
    
    response
}