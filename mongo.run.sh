#!/bin/bash

# Hàm để kiểm tra kết nối MongoDB
check_mongo_connection() {
  echo "Kiểm tra kết nối tới MongoDB..."
  for i in {1..30}; do
    if nc -z localhost 27017; then
      echo "✅ MongoDB đã sẵn sàng!"
      return 0
    fi
    echo "⏳ Đang đợi MongoDB khởi động... ($i/30)"
    sleep 1
  done
  echo "❌ Không thể kết nối tới MongoDB sau 30 giây!"
  return 1
}

# Dừng container MongoDB hiện tại nếu có
echo "🛑 Dừng container MongoDB hiện tại nếu có..."
docker-compose -f docker-compose.mongo.yml down

# Khởi động MongoDB bằng Docker Compose
echo "🚀 Khởi động MongoDB..."
docker-compose -f docker-compose.mongo.yml up -d

# Kiểm tra kết nối MongoDB
check_mongo_connection
if [ $? -ne 0 ]; then
  echo "❌ Không thể kết nối tới MongoDB. Vui lòng kiểm tra lại!"
  exit 1
fi

# Biên dịch và chạy ứng dụng GraphQL Rust
echo "🔨 Biên dịch và chạy ứng dụng GraphQL Rust..."
cargo run

# Bắt tín hiệu SIGINT (Ctrl+C) để dừng ứng dụng và container
trap 'echo "🛑 Đang dừng ứng dụng và container..."; docker-compose -f docker-compose.mongo.yml down; exit 0' SIGINT
