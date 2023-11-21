use actix_web::HttpResponse;
use mongodb::bson::oid::ObjectId;
use crate::application::services::turma_service::TurmaService;
use crate::{infrastructure::database::schemas::turma_schema::{Turma, OptionTurma}, errors::AppError, port::query_filter::QueryOptions};

#[derive(Clone)]
pub struct TurmaController{
    service: Box<TurmaService>
}

impl TurmaController {
    pub fn new(service: Box<TurmaService>) -> Self{
        TurmaController{
            service
        }
    }

    pub async fn get_one(&self, turma: &OptionTurma) -> Result<HttpResponse, AppError> {
        Ok(self.service.get_one(turma).await
            .map(|result| HttpResponse::Ok().json(result))?)
    }
    pub async fn get_all_turma(&self, turma: &OptionTurma, options: QueryOptions) -> Result<HttpResponse, AppError> {
        Ok(self.service.get_all_turma(turma, options).await
            .map(|result| HttpResponse::Ok().json(result))?)
    }
    
    pub async fn create_turma(&self, turma: Box<Turma>) -> Result<HttpResponse, AppError> {
        Ok(self.service.create_turma(turma).await
            .map(|result| {
                if result.is_some() {HttpResponse::Created().json(&Some(result))} else {HttpResponse::Ok().body("")}
            })?)
    }
    
    pub async fn update_turma(&self, turma: Box<OptionTurma>,  aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<HttpResponse, AppError> {
        Ok(self.service.update_turma(
            turma, aluno_id, prof_id
        ).await
            .map(|result| {
                if result.is_some() {HttpResponse::Ok().json(&Some(result))} else {HttpResponse::Ok().body("")}
            })?)
    }
    
    pub async fn delete_turma(&self, aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<HttpResponse, AppError> {
        Ok(self.service.delete_turma(
            aluno_id, prof_id
        ).await
            .map(|result| HttpResponse::NoContent().json(result))?)
    }

}