use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection,
};
use async_trait::async_trait;
use futures::StreamExt;
use std::error::Error;
use std::marker::{Send, Sync};

use crate::base::error::ServiceError;

#[async_trait]
pub trait BaseService<T>
where
    T: serde::de::DeserializeOwned + Unpin + Send + Sync + 'static,
{
    fn get_collection(&self) -> Collection<T>;
    
    async fn find_by_id(&self, id: &str) -> Result<Option<T>, Box<dyn Error>> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| ServiceError::InvalidId(id.to_string()))?;
            
        let result = self.get_collection()
            .find_one(doc! { "_id": object_id }, None)
            .await
            .map_err(|e| Box::new(ServiceError::DatabaseError(e.to_string())) as Box<dyn Error>)?;
            
        Ok(result)
    }
    
    async fn find_many(&self, filter: Document, limit: Option<i64>, skip: Option<i64>) -> Result<Vec<T>, Box<dyn Error>> {
        let mut options = mongodb::options::FindOptions::default();
        
        if let Some(limit_val) = limit {
            options.limit = Some(limit_val);
        }
        
        if let Some(skip_val) = skip {
            options.skip = Some(skip_val.try_into().unwrap_or(0));
        }
        
        let mut cursor = self.get_collection()
            .find(filter, options)
            .await
            .map_err(|e| Box::new(ServiceError::DatabaseError(e.to_string())) as Box<dyn Error>)?;
            
        let mut results = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => results.push(document),
                Err(e) => return Err(Box::new(ServiceError::DatabaseError(e.to_string()))),
            }
        }
            
        Ok(results)
    }
    
    async fn count(&self, filter: Document) -> Result<u64, Box<dyn Error>> {
        let count = self.get_collection()
            .count_documents(filter, None)
            .await
            .map_err(|e| Box::new(ServiceError::DatabaseError(e.to_string())) as Box<dyn Error>)?;
            
        Ok(count)
    }
}
