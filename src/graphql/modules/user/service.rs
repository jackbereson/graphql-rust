use mongodb::{Collection, Database};
use mongodb::bson::{doc, Document, oid::ObjectId};
use std::error::Error;
use futures::StreamExt;
use async_graphql::ID;

use crate::graphql::modules::user::model::User;

pub struct UserService {
    collection: Collection<User>,
}

impl UserService {
    pub fn new(db: &Database) -> Self {
        Self { collection: db.collection("users") }
    }
    
    pub async fn find_all(&self) -> Vec<User> {
        let cursor = self.collection.find(None, None).await.unwrap_or_else(|e| {
            eprintln!("Query error: {}", e);
            panic!("Could not get data from MongoDB");
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
    
    pub async fn create_user(&self, name: String, email: String, age: Option<i32>) -> Result<User, Box<dyn Error>> {
        let user = User::new(name, email, age);
        
        // Directly implement create functionality instead of calling the trait method
        let result = self.collection.insert_one(user.clone(), None).await
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn Error>)?;

        let id = result.inserted_id.as_object_id().unwrap();
        let mut new_user = user.clone();
        new_user.id = Some(ID(id.to_string()));
        Ok(new_user)
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
        
        // Directly implement update functionality
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Invalid ID: {}", id))) as Box<dyn Error>)?;
            
        update_doc.insert("updated_at", mongodb::bson::DateTime::now());
        
        let update = doc! { "$set": update_doc };
        
        let _ = self.collection
            .update_one(doc! { "_id": object_id }, update, None)
            .await
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn Error>)?;
            
        Ok(self.find_by_id(id).await)
    }
    
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, Box<dyn Error>> {
        let filter = doc! { "email": email };
        
        // Directly implement find_many functionality
        let mut options = mongodb::options::FindOptions::default();
        options.limit = Some(1);
        
        let mut cursor = self.collection
            .find(filter, options)
            .await
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn Error>)?;
            
        let mut results = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => results.push(document),
                Err(e) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))),
            }
        }
            
        Ok(results.into_iter().next())
    }
    
    pub async fn count_all(&self) -> Result<u64, Box<dyn Error>> {
        self.collection.count_documents(doc! {}, None).await
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn Error>)
    }
}

// BaseService and CrudService implementations removed to simplify service structure
