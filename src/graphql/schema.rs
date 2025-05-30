use async_graphql::{Schema, EmptySubscription, Object, Context};
use mongodb::Database;
use crate::graphql::modules::user::resolver::{UserQuery, UserMutation};

pub struct DbContext {
    pub db: Database,
}

// Root Query struct combining all module queries
#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn say_hello(&self) -> &str {
        "Hello, Rust GraphQL!"
    }
    
    // Delegate user queries to the UserQuery resolver
    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<crate::graphql::modules::user::model::User>> {
        UserQuery::default().users(ctx).await.map_err(|e| e.into())
    }
    
    async fn user(&self, ctx: &Context<'_>, id: async_graphql::ID) -> Result<Option<crate::graphql::modules::user::model::User>, async_graphql::Error> {
        let result = UserQuery::default().user(ctx, id).await?;
        Ok(result)
    }
}

// Root Mutation struct combining all module mutations
#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // Delegate user mutations to the UserMutation resolver
    async fn create_user(&self, ctx: &Context<'_>, name: String, email: String, age: Option<i32>) -> Result<crate::graphql::modules::user::model::User, String> {
        UserMutation::default().create_user(ctx, name, email, age).await
    }
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(db: Database) -> AppSchema {
    let db_ctx = DbContext { db };
    Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(db_ctx)
        .finish()
}
