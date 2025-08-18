use mongodb::bson::oid::ObjectId;
use crate::application::models::solicitacao::Solicitacao;
use crate::{infrastructure::{repository::solicitacao_repository::SolicitacaoRepository,
    database::schemas::solicitacao_schema::{SolicitacaoSchema, OptionSolicitacaoSchema}}, errors::AppError, port::query_filter::QueryOptions, application::validation::{create_solicitacao::CreateSolicitacaoValidation, update_solicitacao::UpdateSolicitacaoValidation}};

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

    pub async fn get_one(&self, solicitacao: &OptionSolicitacaoSchema) -> Result<Option<Solicitacao>, AppError> {
        Ok(self.repository.get_one(solicitacao).await.map(|op| op.map(Solicitacao::from))?)
    }
    pub async fn get_all_solicitacao(&self, solicitacao: &OptionSolicitacaoSchema, options: QueryOptions) -> Result<Vec<Solicitacao>, AppError> {
        Ok(self.repository.get_all(solicitacao, options).await.map( |item| item.into_iter().map(Solicitacao::from).collect::<Vec<Solicitacao>>())?)
    }
    
    pub async fn create_solicitacao(&self, mut solicitacao: Box<SolicitacaoSchema>) -> Result<Option<Solicitacao>, AppError> {
        solicitacao.status = "pending".to_string();
        CreateSolicitacaoValidation::validate(&mut(solicitacao))?;
        self.repository.create(solicitacao).await
            .map(|opt_sol| opt_sol.map(|sol| Solicitacao::from(*sol)))
    }
    
    pub async fn update_solicitacao(&self, mut solicitacao: Box<OptionSolicitacaoSchema>, aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<Option<Solicitacao>, AppError> {
        UpdateSolicitacaoValidation::validate(&mut(solicitacao))?;
        self.repository.update_one(
            solicitacao, aluno_id, prof_id
        ).await.map(|op|op.map(Solicitacao::from))
    }
    
    pub async fn delete_solicitacao(&self, aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<bool, AppError> {
        Ok(self.repository.delete_one(
            aluno_id, prof_id
        ).await?)
    }

}