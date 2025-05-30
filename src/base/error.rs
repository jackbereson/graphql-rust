use std::error::Error;
use std::fmt;
use async_graphql::ErrorExtensions;

#[derive(Debug)]
pub enum ServiceError {
    NotFound(String),
    InvalidId(String),
    ValidationError(String),
    DatabaseError(String),
    AuthenticationError(String),
    AuthorizationError(String),
    InternalServerError(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ServiceError::InvalidId(id) => write!(f, "Invalid ID: {}", id),
            ServiceError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ServiceError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ServiceError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            ServiceError::AuthorizationError(msg) => write!(f, "Authorization error: {}", msg),
            ServiceError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl Error for ServiceError {}

// Convert service errors to GraphQL errors
// Instead of implementing From, we provide a method to convert to GraphQL error
impl ServiceError {
    pub fn to_graphql_error(&self) -> async_graphql::Error {
        match self {
            ServiceError::NotFound(msg) => {
                async_graphql::Error::new(msg).extend_with(|_, e| {
                    e.set("code", "NOT_FOUND")
                })
            }
            ServiceError::InvalidId(id) => {
                async_graphql::Error::new(format!("Invalid ID: {}", id)).extend_with(|_, e| {
                    e.set("code", "INVALID_ID")
                })
            }
            ServiceError::ValidationError(msg) => {
                async_graphql::Error::new(msg).extend_with(|_, e| {
                    e.set("code", "VALIDATION_ERROR")
                })
            }
            ServiceError::DatabaseError(msg) => {
                async_graphql::Error::new("Database error occurred").extend_with(|_, e| {
                    e.set("code", "DATABASE_ERROR")
                })
            }
            ServiceError::AuthenticationError(msg) => {
                async_graphql::Error::new(msg).extend_with(|_, e| {
                    e.set("code", "AUTHENTICATION_ERROR")
                })
            }
            ServiceError::AuthorizationError(msg) => {
                async_graphql::Error::new(msg).extend_with(|_, e| {
                    e.set("code", "AUTHORIZATION_ERROR")
                })
            }
            ServiceError::InternalServerError(msg) => {
                async_graphql::Error::new("Internal server error").extend_with(|_, e| {
                    e.set("code", "INTERNAL_SERVER_ERROR")
                })
            }
        }
    }
}
