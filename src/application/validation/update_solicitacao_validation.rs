use mongodb::bson::DateTime;

use crate::{errors::AppError, infrastructure::database::schemas::solicitacao_schema::OptionSolicitacao};

pub struct UpdateSolicitacaoValidation{}

impl UpdateSolicitacaoValidation{
    pub fn validate(solicitacao: &mut OptionSolicitacao) -> Result<(),AppError> {
        solicitacao.id_aluno = None;
        solicitacao.id_professor = None;
        solicitacao.created_at = None;
        solicitacao.updated_at = Some(DateTime::now());
        Ok(())
    }
}