use crate::application::models::login::Login;
use crate::di::d_injection::App;
use crate::routes::handler::HANDLER;
use crate::errors::AppError;
use actix_web::{
    web::{Data, Json},
    Responder,
};

pub async fn login(app: Data<App>, query: Json<Login>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.auth;
    let env = &app.env;
    let login = query.into_inner();
    controller
        .login(login, env)
        .await
        .map_err(|err| HANDLER(Box::new(err)))
}
