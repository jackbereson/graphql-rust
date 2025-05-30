//! Cấu hình cơ bản cho tất cả môi trường

use std::env;

/// Cấu trúc chứa các cấu hình cơ bản
#[derive(Debug, Clone)]
pub struct BaseConfig {
    /// Port để chạy server
    pub port: u16,
    /// Host để chạy server
    pub host: String,
    /// URI kết nối MongoDB
    pub mongo_uri: String,
    /// Tên database MongoDB
    pub mongo_db: String,
    /// Môi trường hiện tại
    pub environment: Environment,
}

/// Enum đại diện cho các môi trường khác nhau
#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Production,
    Testing,
}

impl Environment {
    /// Chuyển đổi từ string sang enum Environment
    pub fn from_str(env_str: &str) -> Self {
        match env_str.to_lowercase().as_str() {
            "production" => Environment::Production,
            "testing" => Environment::Testing,
            _ => Environment::Development,
        }
    }
}

impl Default for BaseConfig {
    fn default() -> Self {
        // Lấy thông tin môi trường từ biến môi trường hoặc sử dụng giá trị mặc định
        let environment = env::var("RUST_ENV")
            .unwrap_or_else(|_| "development".to_string());
        
        Self {
            port: 4000,
            host: "127.0.0.1".to_string(),
            mongo_uri: "mongodb://localhost:27017".to_string(),
            mongo_db: "graphql_rust_db".to_string(),
            environment: Environment::from_str(&environment),
        }
    }
}
