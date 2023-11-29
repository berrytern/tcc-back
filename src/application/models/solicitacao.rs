use serde::{Serialize, Deserialize};
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