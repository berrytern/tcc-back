use mongodb::bson::DateTime;
use crate::{errors::AppError, infrastructure::database::schemas::turma_schema::OptionTurma};

pub struct UpdateTurmaValidation{}

impl UpdateTurmaValidation{
    pub fn validate(user: &mut OptionTurma) -> Result<(),AppError> {
        user.id_aluno = None;
        user.id_professor = None;
        user.created_at = None;
        user.updated_at = Some(DateTime::now());
        Ok(())
    }
}