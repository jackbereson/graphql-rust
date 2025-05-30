use mongodb::{Collection, Database};
use mongodb::bson::{doc, Document};
use std::error::Error;
use async_trait::async_trait;
use futures::StreamExt;

use crate::graphql::modules::user::model::User;
use crate::graphql::modules::user::repository::UserRepository;

// Define the BaseService trait locally
#[async_trait]
trait BaseService<T> 
where
    T: serde::de::DeserializeOwned + Unpin + Send + Sync + 'static,
{
    fn get_collection(&self) -> Collection<T>;
    
    async fn find_by_id(&self, id: &str) -> Result<Option<T>, Box<dyn Error>>;
    async fn find_many(&self, filter: Document, limit: Option<i64>, skip: Option<i64>) -> Result<Vec<T>, Box<dyn Error>>;
    async fn count(&self, filter: Document) -> Result<u64, Box<dyn Error>>;
}

// Define the CrudService trait locally
#[async_trait]
trait CrudService<T>: BaseService<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize + Unpin + Send + Sync + 'static,
{
    async fn create(&self, data: T) -> Result<T, Box<dyn Error>>;
    async fn update(&self, id: &str, data: Document) -> Result<Option<T>, Box<dyn Error>>;
    async fn delete(&self, id: &str) -> Result<bool, Box<dyn Error>>;
    async fn delete_many(&self, filter: Document) -> Result<u64, Box<dyn Error>>;
}

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(db: &Database) -> Self {
        let repository = UserRepository::new(db.collection("users"));
        Self { repository }
    }
    
    pub async fn find_all(&self) -> Vec<User> {
        self.repository.find_all().await
    }
    
    pub async fn find_by_id(&self, id: &str) -> Option<User> {
        self.repository.find_by_id(id).await
    }
    
    pub async fn create_user(&self, name: String, email: String, age: Option<i32>) -> Result<User, Box<dyn Error>> {
        let user = User::new(name, email, age);
        match self.repository.create(user).await {
            Ok(user) => Ok(user),
            Err(e) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))
        }
    }
    
    pub async fn update_user(&self, id: &str, name: Option<String>, email: Option<String>, age: Option<i32>) -> Result<Option<User>, Box<dyn Error>> {
        let mut update_doc = Document::new();
        
        if let Some(name) = name {
            update_doc.insert("name", name);
        }
        
        if let Some(email) = email {
            update_doc.insert("email", email);
        }
        
        if let Some(age) = age {
            update_doc.insert("age", age);
        }
        
        self.update(id, update_doc).await
    }
    
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, Box<dyn Error>> {
        let filter = doc! { "email": email };
        let users = self.find_many(filter, Some(1), None).await?;
        Ok(users.into_iter().next())
    }
    
    pub async fn count_all(&self) -> Result<u64, Box<dyn Error>> {
        self.count(doc! {}).await
    }
}

// BaseService implementation
#[async_trait::async_trait]
impl BaseService<User> for UserService {
    fn get_collection(&self) -> Collection<User> {
        self.repository.collection.clone()
    }
    
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, Box<dyn Error>> {
        Ok(self.repository.find_by_id(id).await)
    }
    
    async fn find_many(&self, filter: Document, _limit: Option<i64>, _skip: Option<i64>) -> Result<Vec<User>, Box<dyn Error>> {
        // For simplicity, we're not using the limit and skip parameters in this implementation
        // A more complete implementation would use them in the query
        let cursor = self.get_collection()
            .find(filter, None)
            .await
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn Error>)?;
            
        let users = cursor
            .collect::<Vec<Result<User, _>>>()
            .await
            .into_iter()
            .filter_map(Result::ok)
            .collect();
            
        Ok(users)
    }
    
    async fn count(&self, filter: Document) -> Result<u64, Box<dyn Error>> {
        let count = self.get_collection()
            .count_documents(filter, None)
            .await
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn Error>)?;
            
        Ok(count)
    }
}

// CrudService implementation
#[async_trait::async_trait]
impl CrudService<User> for UserService {
    async fn create(&self, data: User) -> Result<User, Box<dyn Error>> {
        match self.repository.create(data).await {
            Ok(user) => Ok(user),
            Err(e) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))
        }
    }

    async fn update(&self, id: &str, data: Document) -> Result<Option<User>, Box<dyn Error>> {
        // Get the existing user
        let existing_user = self.find_by_id(id).await;
        
        if existing_user.is_none() {
            return Ok(None);
        }

        // Update the document
        let object_id = mongodb::bson::oid::ObjectId::parse_str(id)
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, e)) as Box<dyn Error>)?;
        
        let update_result = self.get_collection()
            .update_one(doc! { "_id": object_id }, doc! { "$set": data }, None)
            .await
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn Error>)?;
        
        if update_result.modified_count == 0 {
            return Ok(existing_user);
        }
        
        // Fetch and return the updated user
        self.find_by_id(id).await.ok_or_else(|| {
            Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "User not found after update")) as Box<dyn Error>
        }).map(Some)
    }

    async fn delete(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        let object_id = mongodb::bson::oid::ObjectId::parse_str(id)
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, e)) as Box<dyn Error>)?;
        
        let delete_result = self.get_collection()
            .delete_one(doc! { "_id": object_id }, None)
            .await
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn Error>)?;
        
        Ok(delete_result.deleted_count > 0)
    }

    async fn delete_many(&self, filter: Document) -> Result<u64, Box<dyn Error>> {
        let delete_result = self.get_collection()
            .delete_many(filter, None)
            .await
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn Error>)?;
        
        Ok(delete_result.deleted_count)
    }
}
