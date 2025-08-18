use mongodb::bson::DateTime;
use crate::application::validation::status_type::StatusType;
use crate::{errors::AppError, infrastructure::database::schemas::solicitacao_schema::SolicitacaoSchema};

pub struct CreateSolicitacaoValidation{}

impl CreateSolicitacaoValidation{
    pub fn validate(solicitacao: &mut SolicitacaoSchema) -> Result<(),AppError> {
        StatusType::validate(&solicitacao.status)?;
        solicitacao.created_at = Some(DateTime::now());
        solicitacao.updated_at = Some(DateTime::now());
        Ok(())
    }
}