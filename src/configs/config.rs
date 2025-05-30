//! API chính để lấy cấu hình ứng dụng

use std::env;
use std::sync::OnceLock;
use dotenv::dotenv;

use crate::configs::base::{BaseConfig, Environment};
use crate::configs::development::DevelopmentConfig;
use crate::configs::production::ProductionConfig;
use crate::configs::testing::TestingConfig;

/// Enum đại diện cho các cấu hình ứng dụng dựa trên môi trường
#[derive(Debug, Clone)]
pub enum AppConfig {
    Development(DevelopmentConfig),
    Production(ProductionConfig),
    Testing(TestingConfig),
}

impl AppConfig {
    /// Lấy cấu hình cơ bản từ bất kỳ cấu hình nào
    pub fn base(&self) -> &BaseConfig {
        match self {
            AppConfig::Development(config) => &config.base,
            AppConfig::Production(config) => &config.base,
            AppConfig::Testing(config) => &config.base,
        }
    }
    
    /// Tạo và lấy cấu hình dựa trên môi trường
    pub fn new() -> Self {
        // Load .env file
        dotenv().ok();
        
        // Lấy môi trường từ biến môi trường hoặc sử dụng giá trị mặc định
        let environment = env::var("RUST_ENV")
            .unwrap_or_else(|_| "development".to_string());
        
        match Environment::from_str(&environment) {
            Environment::Development => AppConfig::Development(DevelopmentConfig::default()),
            Environment::Production => AppConfig::Production(ProductionConfig::default()),
            Environment::Testing => AppConfig::Testing(TestingConfig::default()),
        }
    }
    
    /// Lấy port server
    pub fn port(&self) -> u16 {
        self.base().port
    }
    
    /// Lấy host server
    pub fn host(&self) -> &str {
        &self.base().host
    }
    
    /// Lấy MongoDB URI
    pub fn mongo_uri(&self) -> &str {
        &self.base().mongo_uri
    }
    
    /// Lấy MongoDB database name
    pub fn mongo_db(&self) -> &str {
        &self.base().mongo_db
    }
    
    /// Kiểm tra xem có đang ở môi trường development không
    pub fn is_development(&self) -> bool {
        matches!(self, AppConfig::Development(_))
    }
    
    /// Kiểm tra xem có đang ở môi trường production không
    pub fn is_production(&self) -> bool {
        matches!(self, AppConfig::Production(_))
    }
    
    /// Kiểm tra xem có đang ở môi trường testing không
    pub fn is_testing(&self) -> bool {
        matches!(self, AppConfig::Testing(_))
    }
}

// Singleton để chỉ tạo cấu hình một lần
static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// Lấy cấu hình ứng dụng toàn cục
pub fn get_config() -> &'static AppConfig {
    APP_CONFIG.get_or_init(AppConfig::new)
}
