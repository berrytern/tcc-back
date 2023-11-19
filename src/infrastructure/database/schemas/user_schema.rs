use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::extjson::de::Error,results::InsertOneResult};

// #[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    #[serde(alias = "type")]
    pub user_type: String,
    pub email: String,
    pub matricula: Option<String>,
}
impl Into<User> for InsertOneResult{
    fn into(self) -> User {
        self.into()
    }
    
}
//#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionUser {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "type")]
    pub user_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matricula: Option<String>,
}