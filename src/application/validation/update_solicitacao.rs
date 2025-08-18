use mongodb::bson::DateTime;
use crate::{errors::AppError, infrastructure::database::schemas::solicitacao_schema::OptionSolicitacaoSchema};
use crate::application::validation::status_type::StatusType;
pub struct UpdateSolicitacaoValidation{}

impl UpdateSolicitacaoValidation{
    pub fn validate(solicitacao: &mut OptionSolicitacaoSchema) -> Result<(),AppError> {
        solicitacao.id_aluno = None;
        solicitacao.id_professor = None;
        if let Some(status) = &solicitacao.status {
            StatusType::validate(status)?;
        }
        solicitacao.created_at = None;
        solicitacao.updated_at = Some(DateTime::now().into());
        Ok(())
    }
}