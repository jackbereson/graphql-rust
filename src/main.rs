use axum::{routing::get, Router, response::Html, Extension};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use tokio::net::TcpListener;
use dotenv::dotenv;

mod db;
mod graphql;
mod configs;

use db::connection;
use graphql::schema::{build_schema, AppSchema};
use configs::get_config;

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> Html<String> {
    Html(async_graphql::http::GraphiQLSource::build().endpoint("/graphql").finish())
}

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();
    
    // Kết nối đến MongoDB
    let db = connection::connect_db().await;
    
    // Kiểm tra kết nối
    let _ = connection::test_connection(&db).await;
    
    // Xây dựng schema GraphQL với kết nối MongoDB
    let schema = build_schema(db);
    
    // Lấy cấu hình từ singleton config
    let config = get_config();
    
    let app = Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    // Sử dụng host và port từ cấu hình
    let addr = format!("{}:{}", config.host(), config.port());
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("🚀 GraphQL server running at http://{}:{}/graphql", config.host(), config.port());
    axum::serve(listener, app).await.unwrap();
}
