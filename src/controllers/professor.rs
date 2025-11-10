use crate::application::models::professor::{CreateProfessorModel, ProfessorUpdateModel};
use crate::application::services::professor::ProfessorService;
use crate::{
    errors::AppError,
    infrastructure::database::schemas::user_schema::OptionUserSchema,
    port::query_filter::QueryOptions,
};
use actix_web::HttpResponse;
use mongodb::bson::oid::ObjectId;
#[derive(Clone)]
pub struct ProfessorController {
    service: ProfessorService,
}

impl ProfessorController {
    pub fn new(service: ProfessorService) -> Self {
        ProfessorController { service }
    }

    pub async fn get_one(&self, user: &mut OptionUserSchema) -> Result<HttpResponse, AppError> {
        self
            .service
            .get_one(user)
            .await
            .map(|result| HttpResponse::Ok().json(result))
    }
    pub async fn get_all_professor(
        &self,
        user: &mut OptionUserSchema,
        options: QueryOptions,
    ) -> Result<HttpResponse, AppError> {
        self
            .service
            .get_all_professor(user, options)
            .await
            .map(|result| HttpResponse::Ok().json(result))
    }

    pub async fn create_professor(&self, user: CreateProfessorModel) -> Result<HttpResponse, AppError> {
        self.service.create_professor(user).await.map(|result| {
            if result.is_some() {
                HttpResponse::Created().json(Some(result))
            } else {
                HttpResponse::Ok().body("")
            }
        })
    }

    pub async fn update_professor(
        &self,
        user: ProfessorUpdateModel,
        id: &ObjectId,
    ) -> Result<HttpResponse, AppError> {
        self
            .service
            .update_professor(user, id)
            .await
            .map(|result| {
                if result.is_some() {
                    HttpResponse::Ok().json(Some(result))
                } else {
                    HttpResponse::Ok().body("")
                }
            })
    }

    pub async fn delete_professor(&self, id: &ObjectId) -> Result<HttpResponse, AppError> {
        self
            .service
            .delete_professor(id)
            .await
            .map(|result| HttpResponse::Ok().json(result))
    }
}
