use actix_web::HttpResponse;
use mongodb::bson::oid::ObjectId;
use crate::{infrastructure::{repository::solicitacao_repository::SolicitacaoRepository, database::schemas::solicitacao_schema::{Solicitacao, OptionSolicitacao}}, errors::AppError, port::query_filter::QueryOptions};

#[derive(Clone)]
pub struct SolicitacaoController{
    repository: Box<SolicitacaoRepository>
}

impl SolicitacaoController {
    pub fn new(repository: Box<SolicitacaoRepository>) -> Self {
        SolicitacaoController {
            repository
        }
    }

    pub async fn get_one(&self, solicitacao: &OptionSolicitacao) -> Result<HttpResponse, AppError> {
        Ok(self.repository.get_one(solicitacao).await
            .map(|result| HttpResponse::Ok().json(result))?)
    }
    pub async fn get_all_solicitacao(&self, solicitacao: &OptionSolicitacao, options: QueryOptions) -> Result<HttpResponse, AppError> {
        Ok(self.repository.get_all(solicitacao, options).await
            .map(|result| HttpResponse::Ok().json(result))?)
    }
    
    pub async fn create_solicitacao(&self, solicitacao: Box<Solicitacao>) -> Result<HttpResponse, AppError> {
        Ok(self.repository.create(solicitacao).await
            .map(|result| {
                if result.is_some() {HttpResponse::Created().json(&Some(result))} else {HttpResponse::Ok().body("")}
            })?)
    }
    
    pub async fn update_solicitacao(&self, solicitacao: Box<OptionSolicitacao>, aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<HttpResponse, AppError> {
        Ok(self.repository.update_one(
            solicitacao, aluno_id, prof_id
        ).await
            .map(|result| {
                if result.is_some() {HttpResponse::Ok().json(&Some(result))} else {HttpResponse::Ok().body("")}
            })?)
    }
    
    pub async fn delete_solicitacao(&self, aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<HttpResponse, AppError> {
        Ok(self.repository.delete_one(
            aluno_id, prof_id
        ).await
            .map(|result| HttpResponse::NoContent().json(result))?)
    }

}