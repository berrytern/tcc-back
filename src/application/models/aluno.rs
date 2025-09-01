use mongodb::bson::DateTime;
use serde::{Serialize,Deserialize};
use utoipa::{ToSchema};

use crate::{infrastructure::database::schemas::user_schema::{MyDateTime, OptionUserSchema, UserSchema}, port::query_filter::QueryFilter, utils::regex::{Email, Name, Password}};

#[derive(Serialize,Deserialize,Clone,ToSchema)]
pub struct CreateAlunoModel {
    pub name: Name,
    pub email: Email,
    pub matricula: String,
    pub password: Password,
}
impl From<CreateAlunoModel> for UserSchema {
    fn from(value: CreateAlunoModel) -> Self {
        Self {
            id: None,
            name: value.name.into(),
            email: value.email.into(),
            matricula: Some(value.matricula),
            password: value.password.0,
            user_type: "aluno".to_string(),
            created_at: DateTime::now().into(),
            updated_at: DateTime::now().into(),
        }
    }
}

#[derive(Serialize,Deserialize,Clone,ToSchema)]
pub struct AlunoUpdateModel{
    pub name: Option<String>,
    pub email: Option<Email>,
    pub matricula: Option<String>,
}
impl From<OptionUserSchema> for AlunoUpdateModel {
    fn from(user: OptionUserSchema) -> Self {
        AlunoUpdateModel {
            name: user.name,
            email: user.email.map(|i|i.into()),
            matricula: user.matricula,
        }
    }
}
impl From<AlunoUpdateModel> for OptionUserSchema {
    fn from(user: AlunoUpdateModel) -> Self {
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
pub struct AlunoQueryModel {
    pub name: Option<String>,
    pub email: Option<String>,
    pub matricula: Option<String>,
    pub password: Option<String>,
    pub created_at: Option<MyDateTime>,
    pub updated_at: Option<MyDateTime>,
}

impl From<AlunoQueryModel> for OptionUserSchema {
    fn from(user: AlunoQueryModel) -> Self {
        OptionUserSchema {
            id: None,
            name: user.name,
            email: user.email,
            matricula: user.matricula,
            user_type: Some("aluno".to_string()),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}