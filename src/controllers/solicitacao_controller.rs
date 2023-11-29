use actix_web::HttpResponse;
use mongodb::bson::oid::ObjectId;
use crate::application::services::solicitacao_service::SolicitacaoService;
use crate::{infrastructure::database::schemas::solicitacao_schema::{SolicitacaoSchema, OptionSolicitacaoSchema}, errors::AppError, port::query_filter::QueryOptions};

#[derive(Clone)]
pub struct SolicitacaoController{
    service: Box<SolicitacaoService>
}

impl SolicitacaoController {
    pub fn new(service: Box<SolicitacaoService>) -> Self {
        SolicitacaoController {
            service
        }
    }

    pub async fn get_one(&self, solicitacao: &OptionSolicitacaoSchema) -> Result<HttpResponse, AppError> {
        self.service.get_one(solicitacao).await
            .map(|result| HttpResponse::Ok().json(result))
    }
    pub async fn get_all_solicitacao(&self, solicitacao: &OptionSolicitacaoSchema, options: QueryOptions) -> Result<HttpResponse, AppError> {
        self.service.get_all_solicitacao(solicitacao, options).await
            .map(|result| HttpResponse::Ok().json(result))
    }
    
    pub async fn create_solicitacao(&self, solicitacao: Box<SolicitacaoSchema>) -> Result<HttpResponse, AppError> {
        self.service.create_solicitacao(solicitacao).await
            .map(|result| {
                if result.is_some() {HttpResponse::Created().json(&Some(result))} else {HttpResponse::Ok().body("")}
            })
    }
    
    pub async fn update_solicitacao(&self, solicitacao: Box<OptionSolicitacaoSchema>, aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<HttpResponse, AppError> {
        self.service.update_solicitacao(
            solicitacao, aluno_id, prof_id
        ).await
            .map(|result| {
                if result.is_some() {HttpResponse::Ok().json(&Some(result))} else {HttpResponse::Ok().body("")}
            })
    }
    
    pub async fn delete_solicitacao(&self, aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<HttpResponse, AppError> {
        self.service.delete_solicitacao(
            aluno_id, prof_id
        ).await
            .map(|result| HttpResponse::NoContent().json(result))
    }

}