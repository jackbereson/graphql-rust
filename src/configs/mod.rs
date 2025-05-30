//! Module configs chứa tất cả cấu hình cho ứng dụng

pub mod base;
pub mod development;
pub mod production;
pub mod testing;
pub mod config;

// Re-export các API chính để sử dụng dễ dàng hơn
pub use config::get_config;
