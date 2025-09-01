use std::{fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};
use mongodb::bson::{DateTime,oid::ObjectId};
use utoipa::ToSchema;

// #[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct UserSchema {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MyObjectId>,
    pub name: String,
    pub user_type: String,
    pub email: String,
    pub password: String,
    pub matricula: Option<String>,
    pub created_at: MyDateTime,
    pub updated_at: MyDateTime,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
// This attribute tells utoipa how to generate the schema for MyObjectId.
#[schema(
    // It should be represented as a simple String type in OpenAPI.
    value_type = String, 
    // Add a description for better documentation.
    description = "A 24-character hexadecimal MongoDB Object ID",
    // Provide a realistic example.
    example = "507f191e810c19729de860ea"
)]
pub struct MyObjectId(pub ObjectId);

impl Deref for MyObjectId {
    type Target = ObjectId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// Implement From so you can easily convert an ObjectId into your MyObjectId.
impl From<ObjectId> for MyObjectId {
    fn from(id: ObjectId) -> Self {
        MyObjectId(id)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[schema(
    // It should be represented as a simple String type in OpenAPI.
    value_type = String,
    // Add a description for better documentation.
    description = "A BSON DateTime",
    // Provide a realistic example.
    example = "2021-01-01T00:00:00Z"
)]
pub struct MyDateTime(pub DateTime);


impl Deref for MyDateTime {
    type Target = DateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DateTime> for MyDateTime {
    fn from(dt: DateTime) -> Self {
        MyDateTime(dt)
    }
}

//#[serde(rename_all = "camelCase")]
#[derive( Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct OptionUserSchema {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MyObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matricula: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<MyDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<MyDateTime>,
}
impl Display for OptionUserSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The `write!` macro returns a Result, so we use `?` to handle errors.
        if let Some(id) = &self.id {
            write!(f, "[id:{id:?}]")?;
        }
        if let Some(name) = &self.name {
            write!(f, "[name:{name}]")?;
        }
        if let Some(user_type) = &self.user_type {
            write!(f, "[user_type:{user_type}]")?;
        }
        if let Some(email) = &self.email {
            write!(f, "[email:{email}]")?;
        }
        if let Some(matricula) = &self.matricula {
            write!(f, "[matricula:{matricula}]")?;
        }
        if let Some(created_at) = &self.created_at {
            write!(f, "[created_at:{created_at:?}]")?;
        }
        if let Some(updated_at) = &self.updated_at {
            write!(f, "[updated_at:{updated_at:?}]")?;
        }
        
        Ok(())
    }
}