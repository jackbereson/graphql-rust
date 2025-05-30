//! Cấu hình cho môi trường phát triển

use crate::configs::base::{BaseConfig, Environment};

/// Cấu trúc chứa các cấu hình cho môi trường phát triển
#[derive(Debug, Clone)]
pub struct DevelopmentConfig {
    /// Cấu hình cơ bản
    pub base: BaseConfig,
    /// Debug mode
    pub debug: bool,
}

impl Default for DevelopmentConfig {
    fn default() -> Self {
        let mut base = BaseConfig::default();
        
        // Ghi đè các giá trị mặc định cho môi trường development
        base.environment = Environment::Development;
        base.mongo_db = "graphql_rust_dev_db".to_string();
        
        Self {
            base,
            debug: true,
        }
    }
}
