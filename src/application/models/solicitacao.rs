use std::fmt::Display;

use mongodb::bson::{DateTime};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::infrastructure::database::schemas::user_schema::MyObjectId;
use crate::utils::functions::format_date;
use crate::infrastructure::database::schemas::solicitacao_schema::{SolicitacaoSchema,OptionSolicitacaoSchema};

#[derive(Serialize,Deserialize,Clone)]
pub struct Solicitacao {
    pub id_aluno: Option<String>,
    pub id_professor: Option<String>, 
    pub status: String,
    pub description: String,
    pub comment: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<SolicitacaoSchema> for Solicitacao {
    fn from(value: SolicitacaoSchema) -> Self {
        Self {
            id_aluno: Some(value.id_aluno.to_string()),
            id_professor: Some(value.id_professor.to_string()),
            status: value.status,
            description: value.description,
            comment: value.comment,
            created_at: if let Some(created_at) = value.created_at {format_date(created_at.to_string())} else {"".to_string()},
            updated_at: if let Some(updated_at) = value.updated_at {format_date(updated_at.to_string())} else {"".to_string()},
        }
    }
}
impl From<OptionSolicitacaoSchema> for Solicitacao {
    fn from(value: OptionSolicitacaoSchema) -> Self {
        Self {
            id_aluno: value.id_aluno.map(|id| id.to_string()),
            id_professor: value.id_professor.map(|id| id.to_string()),
            status: if let Some(status) = value.status {status} else {"".to_string()},
            description: if let Some(description) = value.description {description} else {"".to_string()},
            comment: if let Some(comment) = value.comment {comment} else {"".to_string()},
            created_at: if let Some(created_at) = value.created_at {created_at.to_string()} else {"".to_string()},
            updated_at: if let Some(updated_at) = value.updated_at {updated_at.to_string()} else {"".to_string()},
        }
    }
}

#[derive(Serialize,Deserialize,Clone,ToSchema)]
pub struct Pending;
#[derive(Serialize,Deserialize,Clone,ToSchema)]
pub struct Approved;
#[derive(Serialize,Deserialize,Clone,ToSchema)]
pub struct Rejected;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct SolicitacaoStatus<State>{
    state: State
}
impl SolicitacaoStatus<Pending> {
    pub fn approve(self) -> SolicitacaoStatus<Approved> {
        SolicitacaoStatus{state:Approved}
    }
    pub fn reject(self) -> SolicitacaoStatus<Rejected> {
        SolicitacaoStatus{state:Rejected}
    }
}
impl Display for SolicitacaoStatus<Pending> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pending")
    }
}
impl Display for SolicitacaoStatus<Approved> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "approved")
    }
}
impl Display for SolicitacaoStatus<Rejected> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rejected")
    }
}

#[derive(Serialize,Deserialize,Clone,ToSchema)]
pub struct CreateSolicitacaoModel{
    pub id_aluno: MyObjectId,
    pub id_professor: MyObjectId,
    pub description: String,
    pub comment: String,
}
impl From<CreateSolicitacaoModel> for SolicitacaoSchema {
    fn from(value: CreateSolicitacaoModel) -> Self {
        Self {
            id: None,
            id_aluno: value.id_aluno,
            id_professor: value.id_professor,
            status: SolicitacaoStatus{state:Pending}.to_string(),
            description: value.description,
            comment: value.comment,
            created_at: Some(DateTime::now().into()),
            updated_at: Some(DateTime::now().into())
        }
    }
}
#[derive(Serialize,Deserialize,Clone,ToSchema)]
pub enum AnswerStatus {
    Approved(Approved),
    Rejected(Rejected),
}
impl std::fmt::Display for SolicitacaoStatus<AnswerStatus> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Serialize,Deserialize,Clone,ToSchema)]
pub struct UpdateSolicitacaoModel{
    pub status: Option<SolicitacaoStatus<AnswerStatus>>,
    pub description: Option<String>,
    pub comment: Option<String>,
}
impl From<UpdateSolicitacaoModel> for OptionSolicitacaoSchema {
    fn from(value: UpdateSolicitacaoModel) -> Self {
        Self {
            id_aluno: None,
            id_professor: None,
            status: value.status.map(|status| status.to_string()),
            description: value.description,
            comment: value.comment,
            created_at: Some(DateTime::now().into()),
            updated_at: Some(DateTime::now().into())
        }
    }
}