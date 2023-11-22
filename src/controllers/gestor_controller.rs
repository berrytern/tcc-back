use crate::application::services::gestor_service::GestorService;
use crate::{
    errors::AppError,
    infrastructure::database::schemas::user_schema::{OptionUserSchema, UserSchema},
    port::query_filter::QueryOptions,
};
use actix_web::HttpResponse;
use mongodb::bson::oid::ObjectId;
#[derive(Clone)]
pub struct GestorController {
    service: Box<GestorService>,
}

impl GestorController {
    pub fn new(service: Box<GestorService>) -> Self {
        GestorController { service }
    }

    pub async fn get_one(&self, user: &mut OptionUserSchema) -> Result<HttpResponse, AppError> {
        self
            .service
            .get_one(user)
            .await
            .map(|result| HttpResponse::Ok().json(result))
    }
    pub async fn get_all_gestor(
        &self,
        user: &mut OptionUserSchema,
        options: QueryOptions,
    ) -> Result<HttpResponse, AppError> {
        self
            .service
            .get_all_gestor(user, options)
            .await
            .map(|result| HttpResponse::Ok().json(result))
    }

    pub async fn create_gestor(&self, user: Box<UserSchema>) -> Result<HttpResponse, AppError> {
        self.service.create_gestor(user).await.map(|result| {
            if result.is_some() {
                HttpResponse::Created().json(&Some(result))
            } else {
                HttpResponse::Ok().body("")
            }
        })
    }

    pub async fn update_gestor(
        &self,
        user: Box<OptionUserSchema>,
        id: &ObjectId,
    ) -> Result<HttpResponse, AppError> {
        self.service.update_gestor(user, id).await.map(|result| {
            if result.is_some() {
                HttpResponse::Ok().json(&Some(result))
            } else {
                HttpResponse::Ok().body("")
            }
        })
    }

    pub async fn delete_gestor(&self, id: &ObjectId) -> Result<HttpResponse, AppError> {
        self
            .service
            .delete_gestor(id)
            .await
            .map(|result| HttpResponse::Ok().json(result))
    }
}
