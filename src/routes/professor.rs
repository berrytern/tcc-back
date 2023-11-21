use actix_web::{web::{Json,Data,Path,Query}, Responder};
use mongodb::bson::oid::ObjectId;
use crate::{infrastructure::database::schemas::user_schema::{OptionUser, User}, errors::AppError, port::query_filter::QueryFilter};
use crate::di::d_injection::App;
use crate::routes::handler::HANDLER;


pub async fn get_professor(app: Data<App>, query: Query<OptionUser>, id: Path<String>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.professor;
    let mut user = query.into_inner();
    user.id = Some(ObjectId::parse_str(id.into_inner())?);
    controller.get_one(&user).await
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn get_all_professor(app: Data<App>, query: Query<OptionUser>, options: Query<QueryFilter>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.professor;
    let options = options.into_inner();
    let user = query.into_inner();
    controller.get_all_professor(user, options.into()).await
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn create_professor(app: Data<App>, user: Json<User>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.professor;
    controller.create_professor(Box::new(user.into_inner())).await
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn update_professor(app: Data<App>, user: Json<OptionUser>, id: Path<String>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.professor;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller.update_professor(
        Box::new(user.into_inner()), &id
    ).await
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn delete_professor(app: Data<App>, id: Path<String>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.professor;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller.delete_professor(
        &id
    ).await
        .map_err(|err| HANDLER(Box::new(err)))
}