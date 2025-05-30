use mongodb::Collection;
use mongodb::bson::{oid::ObjectId, doc};
use futures::StreamExt;
use async_graphql::ID;

use crate::graphql::modules::user::model::User;

pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(collection: Collection<User>) -> Self {
        Self { collection }
    }

    pub async fn find_all(&self) -> Vec<User> {
        let cursor = self.collection.find(None, None).await.unwrap_or_else(|e| {
            eprintln!("Lỗi truy vấn: {}", e);
            panic!("Không thể lấy dữ liệu từ MongoDB");
        });

        let users: Vec<User> = cursor.collect::<Vec<_>>().await.into_iter()
            .filter_map(Result::ok)
            .collect();

        users
    }

    pub async fn find_by_id(&self, id: &str) -> Option<User> {
        let oid = match ObjectId::parse_str(id) {
            Ok(oid) => oid,
            Err(_) => return None,
        };

        self.collection.find_one(doc! { "_id": oid }, None).await.unwrap_or(None)
    }

    pub async fn create(&self, user: User) -> Result<User, String> {
        let result = self.collection.insert_one(user.clone(), None).await;

        match result {
            Ok(insert_result) => {
                let id = insert_result.inserted_id.as_object_id().unwrap();
                let mut new_user = user.clone();
                new_user.id = Some(ID(id.to_string()));
                Ok(new_user)
            }
            Err(e) => Err(format!("Lỗi thêm user: {}", e)),
        }
    }
}
