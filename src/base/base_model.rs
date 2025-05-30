use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    
    #[serde(default = "default_created_at")]
    pub created_at: DateTime,
    
    #[serde(default = "default_updated_at")]
    pub updated_at: DateTime,
    
    #[serde(default = "default_status")]
    pub status: i32,
}

fn default_created_at() -> DateTime {
    DateTime::now()
}

fn default_updated_at() -> DateTime {
    DateTime::now()
}

fn default_status() -> i32 {
    1 // Assume 1 is active status
}

impl BaseModel {
    pub fn new() -> Self {
        Self {
            id: None,
            created_at: default_created_at(),
            updated_at: default_updated_at(),
            status: default_status(),
        }
    }
}
