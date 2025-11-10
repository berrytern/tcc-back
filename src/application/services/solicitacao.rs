use mongodb::bson::oid::ObjectId;
use crate::application::models::solicitacao::{CreateSolicitacaoModel, Solicitacao, UpdateSolicitacaoModel};
use crate::{infrastructure::{repository::solicitacao_repository::SolicitacaoRepository,
    database::schemas::solicitacao_schema::{SolicitacaoSchema, OptionSolicitacaoSchema}}, errors::AppError, port::query_filter::QueryOptions, application::validation::{create_solicitacao::CreateSolicitacaoValidation, update_solicitacao::UpdateSolicitacaoValidation}};

#[derive(Clone)]
pub struct SolicitacaoService{
    repository: SolicitacaoRepository
}

impl SolicitacaoService {
    pub fn new(repository: SolicitacaoRepository) -> Self {
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

    pub async fn create_solicitacao(&self, solicitacao: CreateSolicitacaoModel) -> Result<Option<Solicitacao>, AppError> {
        let mut solicitacao: SolicitacaoSchema = solicitacao.into();
        match self.repository.create(&mut solicitacao).await {
            Ok(_) => Ok(Some(solicitacao.into())),
            Err(err) => Err(err),
        }
    }

    pub async fn update_solicitacao(&self, mut solicitacao: UpdateSolicitacaoModel, aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<Option<Solicitacao>, AppError> {
        self.repository.update_one(
            solicitacao.into(), aluno_id, prof_id
        ).await.map(|op|op.map(Solicitacao::from))
    }
    
    pub async fn delete_solicitacao(&self, aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<bool, AppError> {
        Ok(self.repository.delete_one(
            aluno_id, prof_id
        ).await?)
    }

}