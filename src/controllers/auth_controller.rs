use crate::application::models::login::Login;
use crate::application::services::auth_service::AuthService;
use crate::errors::AppError;
use crate::utils::settings::Env;
use actix_web::HttpResponse;

#[derive(Clone)]
pub struct AuthController {
    service: Box<AuthService>,
}

impl AuthController {
    pub fn new(service: Box<AuthService>) -> Self {
        AuthController { service }
    }

    pub async fn login(&self, user: Login, env: &Env) -> Result<HttpResponse, AppError> {
        self
            .service
            .login(user, env)
            .await
            .map(|result| HttpResponse::Ok().json(result))
    }
}
