use actix_web::{web::{Json,Data,Path,Query},HttpResponse, Responder};
use mongodb::bson::oid::ObjectId;
use crate::{infrastructure::database::schemas::turma_schema::{OptionTurma, Turma}, errors::AppError, port::query_filter::QueryFilter};
use crate::di::d_injection::App;
use crate::routes::handler::HANDLER;


pub async fn get_one_turma(app: Data<App>, query: Query<OptionTurma>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.turma;
    let turma = query.into_inner();
    repository.get_one(turma).await
        .map(|result| HttpResponse::Ok().json(result))
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn get_all_turma(app: Data<App>, query: Query<OptionTurma>, opt: Query<QueryFilter>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.turma;
    let turma = query.into_inner();
    repository.get_all(turma, opt.into_inner().into()).await
        .map(|result| HttpResponse::Ok().json(result))
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn create_turma(app: Data<App>, turma: Json<Turma>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.turma;
    repository.create(Box::new(turma.into_inner())).await
        .map(|result| {
            if result.is_some() {HttpResponse::Created().json(&Some(result))} else {HttpResponse::Ok().body("")}
        })
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn update_turma(app: Data<App>, turma: Json<OptionTurma>, ids: Path<(String,String)>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.turma;
    let (aluno_id, professor_id) = ids.into_inner();
    let aluno_id = ObjectId::parse_str(&aluno_id)?;
    let professor_id = ObjectId::parse_str(&professor_id)?;
    repository.update_one(
        Box::new(turma.into_inner()), aluno_id,professor_id
    ).await
        .map(|result| {
            if result.is_some() {HttpResponse::Ok().json(&Some(result))} else {HttpResponse::Ok().body("")}
        })
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn delete_turma(app: Data<App>, ids: Path<(String,String)>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.turma;
    let (aluno_id, professor_id) = ids.into_inner();
    let aluno_id = ObjectId::parse_str(&aluno_id)?;
    let professor_id = ObjectId::parse_str(&professor_id)?;
    repository.delete_one(
        aluno_id, professor_id
    ).await
        .map(|result| HttpResponse::Ok().json(result))
        .map_err(|err| HANDLER(Box::new(err)))
}