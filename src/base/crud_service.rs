use mongodb::{
    bson::{doc, oid::ObjectId, Document},
};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;
use std::marker::{Send, Sync};

use crate::base::base_service::BaseService;
use crate::base::error::ServiceError;

#[async_trait]
pub trait CrudService<T>: BaseService<T>
where
    T: DeserializeOwned + Serialize + Unpin + Send + Sync + 'static,
{
    async fn create(&self, data: T) -> Result<T, Box<dyn Error>> {
        let result = self.get_collection()
            .insert_one(data, None)
            .await
            .map_err(|e| Box::new(ServiceError::DatabaseError(e.to_string())) as Box<dyn Error>)?;
            
        self.find_by_id(&result.inserted_id.as_object_id().unwrap().to_string()).await?
            .ok_or_else(|| Box::new(ServiceError::NotFound("Created document not found".to_string())) as Box<dyn Error>)
    }
    
    async fn update(&self, id: &str, data: Document) -> Result<Option<T>, Box<dyn Error>> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| ServiceError::InvalidId(id.to_string()))?;
            
        // Add updated_at field to the update document
        let mut update_doc = data.clone();
        update_doc.insert("updated_at", mongodb::bson::DateTime::now());
        
        let update = doc! { "$set": update_doc };
        
        let _ = self.get_collection()
            .update_one(doc! { "_id": object_id }, update, None)
            .await
            .map_err(|e| Box::new(ServiceError::DatabaseError(e.to_string())) as Box<dyn Error>)?;
            
        self.find_by_id(id).await
    }
    
    async fn delete(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| ServiceError::InvalidId(id.to_string()))?;
            
        let result = self.get_collection()
            .delete_one(doc! { "_id": object_id }, None)
            .await
            .map_err(|e| Box::new(ServiceError::DatabaseError(e.to_string())) as Box<dyn Error>)?;
            
        Ok(result.deleted_count > 0)
    }
    
    async fn delete_many(&self, filter: Document) -> Result<u64, Box<dyn Error>> {
        let result = self.get_collection()
            .delete_many(filter, None)
            .await
            .map_err(|e| Box::new(ServiceError::DatabaseError(e.to_string())) as Box<dyn Error>)?;
            
        Ok(result.deleted_count)
    }
}
