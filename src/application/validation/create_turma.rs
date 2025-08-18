use mongodb::bson::DateTime;

use crate::{errors::AppError, infrastructure::database::schemas::turma_schema::Turma};

pub struct CreateTurmaValidation{}

impl CreateTurmaValidation{
    pub fn validate(solicitacao: &mut Turma) -> Result<(),AppError> {
        solicitacao.created_at = Some(DateTime::now());
        solicitacao.updated_at = Some(DateTime::now());
        Ok(())
    }
}