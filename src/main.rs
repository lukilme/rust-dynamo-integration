mod app;
mod config;
mod db;
mod handlers;
mod models;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use axum::serve;

use app::{build_router, AppState};
use config::AppConfig;
use db::dynamodb::create_client;

use crate::config::RemoteCloud;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
   
    let cfg = AppConfig::from_env();
    println!("{}", cfg);

    let cloud_cfg = RemoteCloud::from_env();
    println!("{}", cloud_cfg);

    let dynamodb = create_client(&cfg).await;

    let state = AppState {
        dynamodb,
        table: cfg.dynamodb_table.clone(),
    };

    let app = build_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = TcpListener::bind(addr)
        .await
        .expect("falha ao bind no endereço");

    println!("Listening on http://{}", addr);

    serve(listener, app).await.unwrap();
}
