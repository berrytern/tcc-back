use mongodb::bson::oid::ObjectId;
use crate::{infrastructure::{repository::solicitacao_repository::SolicitacaoRepository, database::schemas::solicitacao_schema::{Solicitacao, OptionSolicitacao}}, errors::AppError, port::query_filter::QueryOptions, application::validation::{create_solicitacao_validation::CreateSolicitacaoValidation, update_solicitacao_validation::UpdateSolicitacaoValidation}};

#[derive(Clone)]
pub struct SolicitacaoService{
    repository: Box<SolicitacaoRepository>
}

impl SolicitacaoService {
    pub fn new(repository: Box<SolicitacaoRepository>) -> Self {
        SolicitacaoService {
            repository
        }
    }

    pub async fn get_one(&self, solicitacao: &OptionSolicitacao) -> Result<Option<Solicitacao>, AppError> {
        Ok(self.repository.get_one(solicitacao).await?)
    }
    pub async fn get_all_solicitacao(&self, solicitacao: &OptionSolicitacao, options: QueryOptions) -> Result<Vec<Solicitacao>, AppError> {
        Ok(self.repository.get_all(solicitacao, options).await?)
    }
    
    pub async fn create_solicitacao(&self, mut solicitacao: Box<Solicitacao>) -> Result<Option<Box<Solicitacao>>, AppError> {
        CreateSolicitacaoValidation::validate(&mut(*solicitacao))?;
        Ok(self.repository.create(solicitacao).await?)
    }
    
    pub async fn update_solicitacao(&self, mut solicitacao: Box<OptionSolicitacao>, aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<Option<Solicitacao>, AppError> {
        UpdateSolicitacaoValidation::validate(&mut(*solicitacao))?;
        Ok(self.repository.update_one(
            solicitacao, aluno_id, prof_id
        ).await?)
    }
    
    pub async fn delete_solicitacao(&self, aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<bool, AppError> {
        Ok(self.repository.delete_one(
            aluno_id, prof_id
        ).await?)
    }

}