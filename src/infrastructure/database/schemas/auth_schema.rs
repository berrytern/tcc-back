use serde::{Deserialize, Serialize};
use mongodb::bson::{DateTime,oid::ObjectId};

// #[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_type: String,
    pub refresh_token: String,
    pub last_login: DateTime,
}