# GraphQL Rust MongoDB API

API GraphQL đơn giản được xây dựng bằng Rust, với MongoDB làm cơ sở dữ liệu.

## Yêu cầu

- [Rust](https://www.rust-lang.org/tools/install) và Cargo
- [Docker](https://docs.docker.com/get-docker/) và Docker Compose
- [netcat](https://formulae.brew.sh/formula/netcat) (cho script run.sh)

## Cấu hình

Dự án sử dụng file `.env` để quản lý các biến môi trường. Bạn có thể chỉnh sửa file này để thay đổi cấu hình.

### File môi trường

- `.env`: Cấu hình cho môi trường development (mặc định)
- `.env.production`: Cấu hình cho môi trường production
- `.env.testing`: Cấu hình cho môi trường testing

### Chọn môi trường

Để chọn môi trường, bạn có thể đặt biến môi trường `RUST_ENV`:

```bash
# Development (mặc định)
RUST_ENV=development cargo run

# Production
RUST_ENV=production cargo run

# Testing
RUST_ENV=testing cargo run
```

Hoặc bạn có thể sao chép file môi trường tương ứng thành file `.env`:

```bash
# Production
cp .env.production .env

# Testing
cp .env.testing .env
```

## Cài đặt và Chạy

### Cách 1: Sử dụng script tự động

```bash
./run.sh
```

Script này sẽ:
1. Dừng container MongoDB hiện tại (nếu có)
2. Khởi động container MongoDB mới
3. Đợi MongoDB khởi động hoàn tất
4. Biên dịch và chạy ứng dụng GraphQL Rust

### Cách 2: Chạy thủ công

1. Khởi động MongoDB bằng Docker Compose:
```bash
docker-compose up -d
```

2. Biên dịch và chạy ứng dụng:
```bash
cargo run
```

## Sử dụng API GraphQL

Sau khi ứng dụng chạy, bạn có thể truy cập GraphQL Playground tại:
```
http://localhost:4000/graphql
```

### Các truy vấn mẫu

1. Hello World:
```graphql
{
  hello
}
```

2. Lấy danh sách người dùng:
```graphql
{
  users {
    id
    name
    email
    age
  }
}
```

3. Lấy người dùng theo ID:
```graphql
{
  user(id: "ID_của_người_dùng") {
    id
    name
    email
    age
  }
}
```

4. Tạo người dùng mới:
```graphql
mutation {
  createUser(name: "Nguyễn Văn A", email: "nguyenvana@example.com", age: 30) {
    id
    name
    email
    age
  }
}
```

## Cấu hình

Bạn có thể cấu hình kết nối MongoDB qua biến môi trường:

- `MONGODB_URI`: URI kết nối đến MongoDB (mặc định: `mongodb://localhost:27017`)
- `MONGODB_DB`: Tên cơ sở dữ liệu (mặc định: `graphql_rust_db`)
