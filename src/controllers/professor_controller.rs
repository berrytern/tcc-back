use actix_web::HttpResponse;
use mongodb::bson::oid::ObjectId;
use crate::{infrastructure::{repository::professor_repository::ProfessorRepository, database::schemas::user_schema::{User, OptionUser}}, errors::AppError, port::query_filter::QueryOptions};

#[derive(Clone)]
pub struct ProfessorController{
    repository: Box<ProfessorRepository>
}

impl ProfessorController {
    pub fn new(repository: Box<ProfessorRepository>) -> Self{
        ProfessorController{
            repository
        }
    }

    pub async fn get_one(&self, user: &OptionUser) -> Result<HttpResponse, AppError> {
        Ok(self.repository.get_one(user).await
            .map(|result| HttpResponse::Ok().json(result))?)
    }
    pub async fn get_all_professor(&self, user: OptionUser, options: QueryOptions) -> Result<HttpResponse, AppError> {
        Ok(self.repository.get_all(user, options).await
            .map(|result| HttpResponse::Ok().json(result))?)
    }
    
    pub async fn create_professor(&self, user: Box<User>) -> Result<HttpResponse, AppError> {
        Ok(self.repository.create(user).await
            .map(|result| {
                if result.is_some() {HttpResponse::Created().json(&Some(result))} else {HttpResponse::Ok().body("")}
            })?)
    }
    
    pub async fn update_professor(&self, user: Box<OptionUser>, id: &ObjectId) -> Result<HttpResponse, AppError> {
        Ok(self.repository.update_one(
            user, id
        ).await
            .map(|result| {
                if result.is_some() {HttpResponse::Ok().json(&Some(result))} else {HttpResponse::Ok().body("")}
            })?)
    }
    
    pub async fn delete_professor(&self, id: &ObjectId) -> Result<HttpResponse, AppError> {
        Ok(self.repository.delete_one(
            id
        ).await
            .map(|result| HttpResponse::NoContent().json(result))?)
    }

}