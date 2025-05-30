FROM rust:alpine as builder

# Cài đặt dependencies để build
RUN apk add --no-cache musl-dev openssl-dev pkgconfig openssl-libs-static gcc

WORKDIR /usr/src/app

# Create blank project
RUN USER=root cargo new --bin graphql-rust
WORKDIR /usr/src/app/graphql-rust

# Copy manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build dependencies to cache them - sử dụng normal target thay vì musl
RUN cargo build --release
RUN rm src/*.rs

# Copy source code
COPY ./src ./src

# Build the application
RUN touch src/main.rs
RUN cargo build --release

# Đưa vào binary-only container cực nhỏ
FROM alpine:latest as compressor
RUN apk add --no-cache upx
COPY --from=builder /usr/src/app/graphql-rust/target/release/graphql-rust /graphql-rust
RUN upx --best --lzma /graphql-rust

# Runtime stage - dùng alpine để có container siêu nhẹ
FROM alpine:3.18

# Cài đặt SSL certificates và các thư viện cần thiết tối thiểu
RUN apk add --no-cache ca-certificates openssl tzdata && \
    rm -rf /var/cache/apk/*

WORKDIR /usr/local/bin

# Copy only the compressed binary
COPY --from=compressor /graphql-rust .

# Expose the application port
EXPOSE 4000

# Run the binary
CMD ["./graphql-rust"]
