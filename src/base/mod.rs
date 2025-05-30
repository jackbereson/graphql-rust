// Export các thành phần trong module base
pub mod base_model;
pub mod base_service;
pub mod crud_service;
pub mod error;
pub mod base_event;
pub mod base_router;

// Re-export commonly used items
pub use base_model::BaseModel;
pub use error::ServiceError;
