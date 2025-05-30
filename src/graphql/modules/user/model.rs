use async_graphql::{SimpleObject, ID};
use serde::{Serialize, Deserialize};
use mongodb::bson::doc;

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct User {
    // Sử dụng serde để bỏ qua trường này khi nó là None
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id", with = "crate::graphql::modules::user::model::bson_id_as_string")]
    pub id: Option<ID>,
    pub name: String,
    pub email: String,
    pub age: Option<i32>,
}

// Helper module để chuyển đổi ObjectId <-> String
pub mod bson_id_as_string {
    use mongodb::bson::oid::ObjectId;
    use serde::{self, Deserialize, Serializer, Deserializer};
    use async_graphql::ID;

    pub fn serialize<S>(id: &Option<ID>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match id {
            Some(id) => serializer.serialize_str(&id),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<ID>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = Option::<String>::deserialize(deserializer)?;
        match id {
            Some(id) => {
                let oid = ObjectId::parse_str(&id).map_err(serde::de::Error::custom)?;
                Ok(Some(ID(oid.to_string())))
            }
            None => Ok(None),
        }
    }
}

impl User {
    pub fn new(name: String, email: String, age: Option<i32>) -> Self {
        Self {
            id: None,
            name,
            email,
            age,
        }
    }
}
