FROM rust:latest as builder

WORKDIR /usr/src/app

# Create blank project
RUN USER=root cargo new --bin graphql-rust
WORKDIR /usr/src/app/graphql-rust

# Copy manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy source code
COPY ./src ./src

# Build the application
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install OpenSSL and CA certificates
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    openssl \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copy the built binary
COPY --from=builder /usr/src/app/graphql-rust/target/release/graphql-rust .

# Set environment variables
ENV RUST_ENV=production
ENV HOST=0.0.0.0
ENV PORT=4000
ENV MONGO_URI=mongodb://mongodb:27017
ENV MONGO_DB=graphql_rust_db

# Expose the application port
EXPOSE 4000

# Run the binary
CMD ["./graphql-rust"]
