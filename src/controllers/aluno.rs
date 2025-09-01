use crate::application::models::aluno::{AlunoUpdateModel, CreateAlunoModel};
use crate::application::services::aluno::AlunoService;
use crate::{
    errors::AppError,
    infrastructure::database::schemas::user_schema::OptionUserSchema,
    port::query_filter::QueryOptions,
};
use actix_web::http::header;
use actix_web::HttpResponse;
use mongodb::bson::oid::ObjectId;
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;

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
        redis: Option<MultiplexedConnection>,
    ) -> Result<HttpResponse, AppError> {
        let result = self
            .service
            .get_all_aluno(user, options)
            .await?;
        match serde_json::to_string(&result) {
            Ok(result) => {
                if redis.is_some() {
                    redis.unwrap().set_ex(format!("/v1/alunos?{user}"), result.to_string(), 60).await?
                }
                Ok(HttpResponse::Ok().append_header(header::ContentType::json()).body(result))
            },
            _ => Ok(HttpResponse::InternalServerError().body("Failed to serialize response"))
        }
    }

    pub async fn create_aluno(&self, user: CreateAlunoModel) -> Result<HttpResponse, AppError> {
        self.service.create_aluno(user).await.map(|result| {
            if result.is_some() {
                HttpResponse::Created().json(Some(result))
            } else {
                HttpResponse::Ok().body("")
            }
        })
    }

    pub async fn update_aluno(
        &self,
        user: Box<AlunoUpdateModel>,
        id: &ObjectId,
    ) -> Result<HttpResponse, AppError> {
        self.service.update_aluno(user, id).await.map(|result| {
            if result.is_some() {
                HttpResponse::Ok().json(Some(result))
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
