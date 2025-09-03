use mongodb::bson::DateTime;
use serde::{Serialize,Deserialize};
use utoipa::{ToSchema};

use crate::{infrastructure::database::schemas::user_schema::{MyDateTime, MyObjectId, OptionUserSchema, UserSchema}, utils::regex::{Email, Name, Password}};

#[derive(Serialize,Deserialize,Clone,ToSchema)]
pub struct CreateGestorModel {
    pub name: Name,
    pub email: Email,
    pub matricula: String,
    pub password: Password,
}
impl From<CreateGestorModel> for UserSchema {
    fn from(value: CreateGestorModel) -> Self {
        Self {
            id: None,
            name: value.name.into(),
            email: value.email.into(),
            matricula: Some(value.matricula),
            password: value.password.0,
            user_type: "Gestor".to_string(),
            created_at: DateTime::now().into(),
            updated_at: DateTime::now().into(),
        }
    }
}

#[derive(Serialize,Deserialize,Clone,ToSchema)]
pub struct GestorUpdateModel{
    pub name: Option<String>,
    pub email: Option<Email>,
    pub matricula: Option<String>,
}
impl From<OptionUserSchema> for GestorUpdateModel {
    fn from(user: OptionUserSchema) -> Self {
        GestorUpdateModel {
            name: user.name,
            email: user.email.map(|i|i.into()),
            matricula: user.matricula,
        }
    }
}
impl From<GestorUpdateModel> for OptionUserSchema {
    fn from(user: GestorUpdateModel) -> Self {
        OptionUserSchema {
            id: None,
            name: user.name,
            email: user.email.map(|i|i.into()),
            matricula: user.matricula,
            user_type: None,
            created_at: None,
            updated_at: Some(DateTime::now().into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GestorQueryModel {
    pub id: Option<MyObjectId>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub matricula: Option<String>,
    pub password: Option<String>,
    pub created_at: Option<MyDateTime>,
    pub updated_at: Option<MyDateTime>,
}

impl From<GestorQueryModel> for OptionUserSchema {
    fn from(user: GestorQueryModel) -> Self {
        OptionUserSchema {
            id: user.id,
            name: user.name,
            email: user.email,
            matricula: user.matricula,
            user_type: Some("gestor".to_string()),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}