//! Cấu hình cho môi trường sản phẩm

use std::env;
use crate::configs::base::{BaseConfig, Environment};

/// Cấu trúc chứa các cấu hình cho môi trường sản phẩm
#[derive(Debug, Clone)]
pub struct ProductionConfig {
    /// Cấu hình cơ bản
    pub base: BaseConfig,
    /// Bật/tắt caching
    pub enable_cache: bool,
    /// Thời gian cache hết hạn (seconds)
    pub cache_expiration: u64,
}

impl Default for ProductionConfig {
    fn default() -> Self {
        let mut base = BaseConfig::default();
        
        // Ghi đè các giá trị mặc định cho môi trường production
        base.environment = Environment::Production;
        base.host = "0.0.0.0".to_string(); // Bind to all interfaces in production
        base.port = env::var("PORT")
            .ok()
            .and_then(|port| port.parse().ok())
            .unwrap_or(4000);
        base.mongo_uri = env::var("MONGODB_URI")
            .unwrap_or_else(|_| "mongodb://mongo:27017".to_string());
        base.mongo_db = env::var("MONGODB_DB")
            .unwrap_or_else(|_| "graphql_rust_prod_db".to_string());
        
        Self {
            base,
            enable_cache: true,
            cache_expiration: 3600, // 1 hour
        }
    }
}
