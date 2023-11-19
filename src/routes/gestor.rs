use std::{str::FromStr};


use actix_web::{web::{Json,Data,Path,Query},HttpRequest,HttpResponse, Responder};
use mongodb::bson::{oid::ObjectId, doc};
use crate::{infrastructure::database::schemas::user_schema::{OptionUser, User}, errors::AppError};
use crate::di::d_injection::App;
use crate::routes::handler::HANDLER;


pub async fn get_gestor(app: Data<App>, query: Query<OptionUser>, id: Path<ObjectId>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.gestor;
    let mut user = query.into_inner();
    user.id = Some(id.into_inner()); 
    repository.get_one(user).await
        .map(|result| HttpResponse::Ok().json(result))
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn get_all_gestor(app: Data<App>, query: Query<OptionUser>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.gestor;
    let user = query.into_inner();
    repository.get_all(user).await
        .map(|result| HttpResponse::Ok().json(result))
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn create_gestor(app: Data<App>, user: Json<User>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.gestor;
    repository.create(Box::new(user.into_inner())).await
        .map(|result| {
            if result.is_some() {HttpResponse::Created().json(&Some(result))} else {HttpResponse::Ok().body("")}
        })
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn update_gestor(app: Data<App>, user: Json<OptionUser>, id: Path<ObjectId>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.gestor;
    repository.update_one(
        &user.into_inner(), id.into_inner()
    ).await
        .map(|result| {
            if result.is_some() {HttpResponse::Ok().json(&Some(result))} else {HttpResponse::Ok().body("")}
        })
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn delete_gestor(app: Data<App>, id: Path<ObjectId>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.gestor;
    repository.delete_one(
        id.into_inner()
    ).await
        .map(|result| HttpResponse::Ok().json(result))
        .map_err(|err| HANDLER(Box::new(err)))
}