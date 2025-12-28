use aws_sdk_dynamodb::{Client, config::Builder as DynConfigBuilder};
use aws_credential_types;
use crate::config;

pub async fn create_client(config: &config::AppConfig) -> Client {
    print!("\nstate:{}\n",config.dynamodb_local);
    if config.dynamodb_local {
        create_local_client(
            config.dynamodb_endpoint
                .as_deref()
                .unwrap_or("http://localhost:8000")
        ).await
    } else {
        create_remote_client(config).await
    }
}

pub async fn create_remote_client(config: &config::AppConfig) -> Client {
    let mut loader = aws_config::defaults(aws_config::BehaviorVersion::latest());
    
    if !config.remote_cloud_config.aws_region.is_empty() {
        loader = loader.region(
            aws_sdk_dynamodb::config::Region::new(
                config.remote_cloud_config.aws_region.clone()
            )
        );
    }
    
    if let (Some(access_key), Some(secret_key)) = (
        &config.remote_cloud_config.aws_access_key_id,
        &config.remote_cloud_config.aws_secret_access_key,
    ) {
        let credentials = aws_credential_types::Credentials::new(
            access_key,
            secret_key,
            None,
            None,
            "app-config",
        );
        loader = loader.credentials_provider(credentials);
    }
    
    let shared_config = loader.load().await;
    Client::new(&shared_config)
}

pub async fn create_local_client(endpoint: &str) -> Client {
    let shared_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .test_credentials()
        .endpoint_url(endpoint)
        .load()
        .await;

    let dynamo_config = DynConfigBuilder::from(&shared_config).build();
    Client::from_conf(dynamo_config)
}