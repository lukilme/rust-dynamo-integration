use std::env;
use tracing_subscriber::{fmt, EnvFilter};
use tracing::{info, error, warn, debug, Level};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

pub fn init_logging() {
    let log_level = env::var("LOG_LEVEL")
        .unwrap_or_else(|_| "info".to_string())
        .to_lowercase();

    let log_to_file = env::var("LOG_TO_FILE")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase() == "true";
    
    let is_production = env::var("RUST_ENV")
        .unwrap_or_else(|_| "development".to_string())
        .to_lowercase() == "production";
    
    if log_to_file {
        let file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            "logs",
            "app.log"
        );
        
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        
        if is_production {
            tracing_subscriber::fmt()
                .with_env_filter(&log_level)
                .json()
                .with_writer(non_blocking)
                .init();
        } else {
            tracing_subscriber::fmt()
                .with_env_filter(&log_level)
                .with_target(false)
                .compact()
                .with_writer(non_blocking)
                .init();
        }
    } else {
        if is_production {
            tracing_subscriber::fmt()
                .with_env_filter(&log_level)
                .json()
                .init();
        } else {
            tracing_subscriber::fmt()
                .with_env_filter(&log_level)
                .with_target(false)
                .compact()
                .init();
        }
    }
    
    info!("Logging initialized at {} level", log_level);
}


#[macro_export]
macro_rules! log_error {
    ($error:expr, $msg:expr) => {
        tracing::error!(error = %$error, $msg);
    };
    ($error:expr, $msg:expr, $($field:tt)*) => {
        tracing::error!(error = %$error, $msg, $($field)*);
    };
}


#[macro_export]
macro_rules! log_event {
    ($event:expr, $($field:tt)*) => {
        tracing::info!(event = $event, $($field)*);
    };
}

#[macro_export]
macro_rules! log_perf {
    ($operation:expr, $duration_ms:expr) => {
        tracing::debug!(operation = $operation, duration_ms = $duration_ms);
    };
}