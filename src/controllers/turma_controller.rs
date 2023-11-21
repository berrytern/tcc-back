use actix_web::HttpResponse;
use mongodb::bson::oid::ObjectId;
use crate::{infrastructure::{repository::turma_repository::TurmaRepository, database::schemas::turma_schema::{Turma, OptionTurma}}, errors::AppError, port::query_filter::QueryOptions};

#[derive(Clone)]
pub struct TurmaController{
    repository: Box<TurmaRepository>
}

impl TurmaController {
    pub fn new(repository: Box<TurmaRepository>) -> Self{
        TurmaController{
            repository
        }
    }

    pub async fn get_one(&self, turma: &OptionTurma) -> Result<HttpResponse, AppError> {
        Ok(self.repository.get_one(turma).await
            .map(|result| HttpResponse::Ok().json(result))?)
    }
    pub async fn get_all_turma(&self, turma: &OptionTurma, options: QueryOptions) -> Result<HttpResponse, AppError> {
        Ok(self.repository.get_all(turma, options).await
            .map(|result| HttpResponse::Ok().json(result))?)
    }
    
    pub async fn create_turma(&self, turma: Box<Turma>) -> Result<HttpResponse, AppError> {
        Ok(self.repository.create(turma).await
            .map(|result| {
                if result.is_some() {HttpResponse::Created().json(&Some(result))} else {HttpResponse::Ok().body("")}
            })?)
    }
    
    pub async fn update_turma(&self, turma: Box<OptionTurma>,  aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<HttpResponse, AppError> {
        Ok(self.repository.update_one(
            turma, aluno_id, prof_id
        ).await
            .map(|result| {
                if result.is_some() {HttpResponse::Ok().json(&Some(result))} else {HttpResponse::Ok().body("")}
            })?)
    }
    
    pub async fn delete_turma(&self, aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<HttpResponse, AppError> {
        Ok(self.repository.delete_one(
            aluno_id, prof_id
        ).await
            .map(|result| HttpResponse::NoContent().json(result))?)
    }

}