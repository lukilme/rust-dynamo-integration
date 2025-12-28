use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// DTO para criação (recebido do cliente)
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    
    #[validate(email)]
    pub email: String,
    
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<CreateUserRequest> for User {
    fn from(req: CreateUserRequest) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: req.name,
            email: req.email,
            password: req.password,
            created_at: chrono::Utc::now(),
        }
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
        }
    }
}