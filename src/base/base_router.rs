use actix_web::{web, HttpResponse, Scope, Error as ActixError};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::graphql::schema::AppSchema;

// Base router trait
pub trait BaseRouter {
    fn configure() -> Scope;
}

// GraphQL router implementation
pub struct GraphQLRouter;

impl GraphQLRouter {
    pub async fn graphql_handler(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
        schema.execute(req.into_inner()).await.into()
    }
    
    pub async fn graphql_playground() -> Result<HttpResponse, ActixError> {
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql"))))
    }
}

impl BaseRouter for GraphQLRouter {
    fn configure() -> Scope {
        web::scope("/graphql")
            .route("", web::post().to(Self::graphql_handler))
            .route("", web::get().to(Self::graphql_playground))
    }
}

// Health check router
pub struct HealthCheckRouter;

async fn health_handler() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

impl BaseRouter for HealthCheckRouter {
    fn configure() -> Scope {
        web::scope("/health")
            .route("", web::get().to(health_handler))
    }
}
