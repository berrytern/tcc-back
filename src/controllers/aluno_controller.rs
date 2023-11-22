use crate::application::services::aluno_service::AlunoService;
use crate::{
    errors::AppError,
    infrastructure::database::schemas::user_schema::{OptionUserSchema, UserSchema},
    port::query_filter::QueryOptions,
};
use actix_web::HttpResponse;
use mongodb::bson::oid::ObjectId;
#[derive(Clone)]
pub struct AlunoController {
    service: Box<AlunoService>,
}

impl AlunoController {
    pub fn new(service: Box<AlunoService>) -> Self {
        AlunoController { service }
    }

    pub async fn get_one(&self, user: &mut OptionUserSchema) -> Result<HttpResponse, AppError> {
        self
            .service
            .get_one(user)
            .await
            .map(|result| HttpResponse::Ok().json(result))
    }
    pub async fn get_all_aluno(
        &self,
        user: &mut OptionUserSchema,
        options: QueryOptions,
    ) -> Result<HttpResponse, AppError> {
        self
            .service
            .get_all_aluno(user, options)
            .await
            .map(|result| HttpResponse::Ok().json(result))
    }

    pub async fn create_aluno(&self, user: Box<UserSchema>) -> Result<HttpResponse, AppError> {
        self.service.create_aluno(user).await.map(|result| {
            if result.is_some() {
                HttpResponse::Created().json(&Some(result))
            } else {
                HttpResponse::Ok().body("")
            }
        })
    }

    pub async fn update_aluno(
        &self,
        user: Box<OptionUserSchema>,
        id: &ObjectId,
    ) -> Result<HttpResponse, AppError> {
        self.service.update_aluno(user, id).await.map(|result| {
            if result.is_some() {
                HttpResponse::Ok().json(&Some(result))
            } else {
                HttpResponse::Ok().body("")
            }
        })
    }

    pub async fn delete_aluno(&self, id: &ObjectId) -> Result<HttpResponse, AppError> {
        self
            .service
            .delete_aluno(id)
            .await
            .map(|result| HttpResponse::Ok().json(result))
    }
}
