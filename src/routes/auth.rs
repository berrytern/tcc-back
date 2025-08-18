use crate::application::models::{access_token::AccessToken, login::Login};
use crate::di::d_injection::App as DI_APP;
use crate::errors::AppError;
use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};


#[utoipa::path(tag = "auth", responses((status = OK, body = AccessToken)))]
#[post("/v1/auth")]
pub async fn login(app: Data<DI_APP>, query: Json<Login>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.auth;
    let env = &app.env;
    let login = query.into_inner();
    controller
        .login(login, env)
        .await
}
