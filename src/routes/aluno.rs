use actix_web::{web::{Json,Data,Path,Query},HttpResponse, Responder};
use mongodb::bson::oid::ObjectId;
use crate::{infrastructure::database::schemas::user_schema::{OptionUser, User}, errors::AppError, port::query_filter::QueryFilter};
use crate::di::d_injection::App;
use crate::routes::handler::HANDLER;


pub async fn get_aluno(app: Data<App>, query: Query<OptionUser>, id: Path<String>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.aluno;
    let mut user = query.into_inner();
    user.id = Some(ObjectId::parse_str(id.into_inner())?);
    repository.get_one(user).await
        .map(|result| HttpResponse::Ok().json(result))
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn get_all_aluno(app: Data<App>, query: Query<OptionUser>, opt: Query<QueryFilter>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.aluno;
    let user = query.into_inner();
    repository.get_all(user, opt.into_inner().into()).await
        .map(|result| HttpResponse::Ok().json(result))
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn create_aluno(app: Data<App>, user: Json<User>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.aluno;
    repository.create(Box::new(user.into_inner())).await
        .map(|result| {
            if result.is_some() {HttpResponse::Created().json(&Some(result))} else {HttpResponse::Ok().body("")}
        })
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn update_aluno(app: Data<App>, user: Json<OptionUser>, id: Path<String>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.aluno;
    let id = ObjectId::parse_str(id.into_inner())?;
    repository.update_one(
        Box::new(user.into_inner()), id
    ).await
        .map(|result| {
            if result.is_some() {HttpResponse::Ok().json(&Some(result))} else {HttpResponse::Ok().body("")}
        })
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn delete_aluno(app: Data<App>, id: Path<String>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.aluno;
    let id = ObjectId::parse_str(id.into_inner())?;
    repository.delete_one(
        id
    ).await
        .map(|result| HttpResponse::Ok().json(result))
        .map_err(|err| HANDLER(Box::new(err)))
}