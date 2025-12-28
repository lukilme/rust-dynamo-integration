mod app;
mod config;
mod db;
mod handlers;
mod models;
mod logging;
mod middlewares;
mod error;  
use std::net::SocketAddr;
use tokio::net::TcpListener;
use axum::{middleware as other_middleware, serve};

use app::{build_router, AppState};
use config::AppConfig;
use db::dynamodb::create_client;

use crate::config::RemoteCloud;
use tracing::{info, warn, error};


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    logging::init_logging();
    
    let cfg = AppConfig::from_env();
    info!("{}", cfg);
    
    let cloud_cfg = RemoteCloud::from_env();
    info!("{}", cloud_cfg);
    
    let dynamodb = create_client(&cfg).await;
    
    let state = AppState {
        dynamodb,
        table: cfg.dynamodb_table.clone(),
    };
    
    let app = build_router(state).layer(other_middleware::from_fn(middlewares::logging::request_logger));
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    
    let listener = TcpListener::bind(addr).await.expect("falha ao bind no endereço");

    info!("Listening on http://{}", addr);

    serve(listener, app).await.unwrap();
}
