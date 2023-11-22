use serde::{Deserialize, Serialize};
use mongodb::bson::{DateTime,oid::ObjectId};

// #[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSchema {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub user_type: String,
    pub email: String,
    pub password: String,
    pub matricula: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
//#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionUserSchema {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matricula: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime>,
}