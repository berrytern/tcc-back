use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::utils::functions::format_date;
use crate::infrastructure::database::schemas::user_schema::{MyDateTime, UserSchema};

#[derive(Serialize,Deserialize,Clone,ToSchema)]
pub struct UserInput {
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matricula: Option<String>,
}


impl UserInput {
    pub fn into_schema(&self, user_type: &str, created_at: MyDateTime, updated_at: MyDateTime) -> UserSchema {
        UserSchema {
            id: None,
            name: self.name.to_owned(),
            password: self.password.to_owned(),
            user_type: user_type.to_owned(),
            email: self.email.to_owned(),
            matricula: self.matricula.to_owned(),
            created_at,
            updated_at,
        }
    }
}

#[derive(Serialize,Deserialize,Clone,ToSchema)]
pub struct UserOutput {
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
impl From<UserSchema> for UserOutput {
    fn from(value: UserSchema) -> Self {
        Self {
            id: value.id.map(|id| id.to_string()),
            name: value.name,
            user_type: value.user_type,
            email: value.email,
            matricula: value.matricula,
            created_at: format_date(value.created_at.to_string()),
            updated_at: format_date(value.updated_at.to_string()),
        }
    }
}