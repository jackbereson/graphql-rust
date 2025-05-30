use mongodb::{Client, Database, options::ClientOptions};
use crate::configs::get_config;

pub async fn connect_db() -> Database {
    // Lấy cấu hình từ singleton config
    let config = get_config();
    
    // Lấy MongoDB URI từ cấu hình
    let mongo_uri = config.mongo_uri();
    
    // Tạo client options từ connection string
    let client_options = ClientOptions::parse(mongo_uri).await
        .expect("Không thể phân tích cú pháp URI MongoDB");
    
    // Tạo client
    let client = Client::with_options(client_options)
        .expect("Không thể kết nối đến MongoDB");
    
    // Lấy database từ cấu hình
    let db_name = config.mongo_db();
    let db = client.database(db_name);
    
    println!("Đã kết nối thành công đến MongoDB!");
    db
}

// Hàm kiểm tra kết nối
pub async fn test_connection(db: &Database) -> bool {
    match db.list_collection_names(None).await {
        Ok(_) => {
            println!("Kết nối MongoDB hoạt động tốt!");
            true
        },
        Err(e) => {
            eprintln!("Lỗi kết nối MongoDB: {}", e);
            false
        }
    }
}
