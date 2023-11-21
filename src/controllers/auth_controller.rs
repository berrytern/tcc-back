use crate::application::models::login::Login;
use crate::application::services::auth_service::AuthService;
use crate::{
    errors::AppError,
    infrastructure::database::schemas::user_schema::{OptionUser, User},
    port::query_filter::QueryOptions,
};
use actix_web::HttpResponse;
use mongodb::bson::oid::ObjectId;
#[derive(Clone)]
pub struct AuthController {
    service: Box<AuthService>,
}

impl AuthController {
    pub fn new(service: Box<AuthService>) -> Self {
        AuthController { service }
    }

    pub async fn login(&self, user: Login, secret: &str) -> Result<HttpResponse, AppError> {
        Ok(self
            .service
            .login(user, secret)
            .await
            .map(|result| HttpResponse::Ok().json(result))?)
    }
}
