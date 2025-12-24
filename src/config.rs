use std::{env, fmt};

#[derive(Clone)]
pub struct AppConfig {
    pub dynamodb_table: String,
    pub dynamodb_local: bool,
    pub dynamodb_endpoint: Option<String>,
    pub remote_cloud_config: RemoteCloud,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            dynamodb_table: env::var("DYNAMODB_TABLE_NAME_FAKE")
                .unwrap_or_else(|_| "users".into()),
            dynamodb_local: env::var("DYNAMODB_LOCAL_FAKE")
                .unwrap_or_default() == "true",
            dynamodb_endpoint: env::var("DYNAMODB_LOCAL_ENDPOINT_FAKE").ok(),
            remote_cloud_config: RemoteCloud::from_env(),
        }
    }
}

impl fmt::Display for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Table: {}, Local: {}, Endpoint: {:?}",
            self.dynamodb_table,
            self.dynamodb_local,
            self.dynamodb_endpoint
        )
    }
}

#[derive(Clone)]
pub struct RemoteCloud {
    pub aws_access_key_id: Option<String>,
    pub aws_secret_access_key: Option<String>,
    pub aws_region: String,
}

impl RemoteCloud {
    pub fn from_env() -> Self {
        Self {
            aws_access_key_id: env::var("AWS_ACCESS_KEY_ID").ok(),
            aws_secret_access_key: env::var("AWS_SECRET_ACCESS_KEY").ok(),
            aws_region: env::var("AWS_REGION")
                .unwrap_or_else(|_| "us-east-1".into()),
        }
    }
}

impl fmt::Display for RemoteCloud {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "aws_access_key_id: {}\naws_secret_access_key: {}\naws_region: {}",
            self.aws_access_key_id
                .clone()
                .unwrap_or_else(|| "Not set".to_string()),
            self.aws_secret_access_key
                .clone()
                .unwrap_or_else(|| "Not set".to_string()),
            self.aws_region
        )
    }
}