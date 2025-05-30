use async_graphql::{Context, Object, ID, Result};
use crate::graphql::schema::DbContext;
use crate::graphql::modules::user::model::User;
use crate::graphql::modules::user::service::UserService;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    pub async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>, async_graphql::Error> {
        let db_ctx = ctx.data::<DbContext>().expect("Cannot get DB context");
        let user_service = UserService::new(&db_ctx.db);
        Ok(user_service.find_all().await)
    }
    
    pub async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<Option<User>, async_graphql::Error> {
        let db_ctx = ctx.data::<DbContext>().expect("Cannot get DB context");
        let user_service = UserService::new(&db_ctx.db);
        Ok(user_service.find_by_id(&id.to_string()).await)
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(&self, ctx: &Context<'_>, name: String, email: String, age: Option<i32>) -> Result<User, String> {
        let db_ctx = ctx.data::<DbContext>().expect("Cannot get DB context");
        let user_service = UserService::new(&db_ctx.db);
        user_service.create(name, email, age).await
    }
}
