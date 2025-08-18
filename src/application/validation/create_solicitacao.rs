use crate::application::validation::status_type::StatusType;
use crate::{errors::AppError, infrastructure::database::schemas::solicitacao_schema::SolicitacaoSchema};

pub struct CreateSolicitacaoValidation{}

impl CreateSolicitacaoValidation{
    pub fn validate(solicitacao: &mut SolicitacaoSchema) -> Result<(),AppError> {
        StatusType::validate(&solicitacao.status)?;
        Ok(())
    }
}