use serde::{Deserialize, Serialize};
use mongodb::bson::{DateTime,oid::ObjectId};

use crate::errors::{AppError, AppErrorType};

pub struct StatusType {}
impl StatusType {
    pub fn validate(value: &str) -> Result<&str, AppError> {
        match value {
            "pending" => Ok("pending"),
            "accepted" => Ok("accepted"),
            _ => Err(AppError{
                cause: None,
                message: Some(format!("invalid status value:{}; must be one of [pending,accepted]", value)),
                error_type: AppErrorType::ValidationError
            })
        }
    }
}

// #[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SolicitacaoSchema {
    pub id_aluno: ObjectId,
    pub id_professor: ObjectId, 
    pub status: String,
    pub description: String,
    pub comment: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
//#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionSolicitacaoSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_aluno: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_professor: Option<ObjectId>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime>,
}