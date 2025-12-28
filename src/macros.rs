#[macro_export]
macro_rules! api_endpoint {
    ($method:ident, $path:expr, $handler:ident) => {
        Router::new().route($path, axum::routing::$method($handler))
    };
    
    ($method:ident, $path:expr, $handler:ident, $middleware:expr) => {
        Router::new()
            .route($path, axum::routing::$method($handler))
            .layer($middleware)
    };
}