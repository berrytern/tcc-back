use mongodb::bson::oid::ObjectId;
use crate::{infrastructure::{database::schemas::turma_schema::{Turma, OptionTurma}, repository::turma_repository::TurmaRepository}, errors::AppError, port::query_filter::QueryOptions, application::validation::{update_turma_validation::UpdateTurmaValidation, create_turma_validation::CreateTurmaValidation}};

#[derive(Clone)]
pub struct TurmaService{
    repository: Box<TurmaRepository>
}

impl TurmaService {
    pub fn new(repository: Box<TurmaRepository>) -> Self{
        TurmaService{
            repository
        }
    }

    pub async fn get_one(&self, turma: &OptionTurma) -> Result<Option<Turma>, AppError> {
        Ok(self.repository.get_one(turma).await?)
    }
    pub async fn get_all_turma(&self, turma: &OptionTurma, options: QueryOptions) -> Result<Vec<Turma>, AppError> {
        Ok(self.repository.get_all(turma, options).await?)
    }
    
    pub async fn create_turma(&self, mut turma: Box<Turma>) -> Result<Option<Box<Turma>>, AppError> {
        CreateTurmaValidation::validate(&mut(*turma))?;
        Ok(self.repository.create(turma).await?)
    }
    
    pub async fn update_turma(&self, mut turma: Box<OptionTurma>,  aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<Option<Turma>, AppError> {
        UpdateTurmaValidation::validate(&mut(*turma))?;
        Ok(self.repository.update_one(
            turma, aluno_id, prof_id
        ).await?)
    }
    
    pub async fn delete_turma(&self, aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<bool, AppError> {
        Ok(self.repository.delete_one(
            aluno_id, prof_id
        ).await?)
    }

}