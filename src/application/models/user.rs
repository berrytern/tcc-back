use serde::{Serialize, Deserialize};
use crate::utils::functions::format_date;
use crate::infrastructure::database::schemas::user_schema::{UserSchema,OptionUserSchema};

#[derive(Serialize,Deserialize,Clone)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(alias = "type")]
    pub user_type: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matricula: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<UserSchema> for User {
    fn from(value: UserSchema) -> Self {
        Self {
            id: value.id.map(|id| id.to_string()),
            name: value.name,
            user_type: value.user_type,
            email: value.email,
            matricula: value.matricula,
            created_at: if let Some(created_at) = value.created_at {format_date(created_at.to_string())} else {"".to_string()},
            updated_at: if let Some(updated_at) = value.updated_at {format_date(updated_at.to_string())} else {"".to_string()},
        }
    }
}
impl From<OptionUserSchema> for User {
    fn from(value: OptionUserSchema) -> Self {
        Self {
            id: value.id.map(|id| id.to_string()),
            name: if let Some(name) = value.name {name} else {"".to_string()},
            user_type: if let Some(user_type) = value.user_type {user_type} else {"".to_string()},
            email: if let Some(email) = value.email {email} else {"".to_string()},
            matricula: value.matricula,
            created_at: if let Some(created_at) = value.created_at {created_at.to_string()} else {"".to_string()},
            updated_at: if let Some(updated_at) = value.updated_at {updated_at.to_string()} else {"".to_string()},
        }
    }
}