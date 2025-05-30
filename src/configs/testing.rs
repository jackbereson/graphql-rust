//! Cấu hình cho môi trường kiểm thử

use crate::configs::base::{BaseConfig, Environment};

/// Cấu trúc chứa các cấu hình cho môi trường kiểm thử
#[derive(Debug, Clone)]
pub struct TestingConfig {
    /// Cấu hình cơ bản
    pub base: BaseConfig,
    /// Mock services flag
    pub use_mock_services: bool,
}

impl Default for TestingConfig {
    fn default() -> Self {
        let mut base = BaseConfig::default();
        
        // Ghi đè các giá trị mặc định cho môi trường testing
        base.environment = Environment::Testing;
        base.mongo_db = "graphql_rust_test_db".to_string();
        
        Self {
            base,
            use_mock_services: true,
        }
    }
}
