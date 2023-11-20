use actix_web::{web::{Json,Data,Path,Query},HttpResponse, Responder};
use mongodb::bson::oid::ObjectId;
use crate::{infrastructure::database::schemas::solicitacao_schema::{OptionSolicitacao, Solicitacao}, errors::AppError, port::query_filter::QueryFilter};
use crate::di::d_injection::App;
use crate::routes::handler::HANDLER;


pub async fn get_one_solicitacao(app: Data<App>, query: Query<OptionSolicitacao>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.solicitacao;
    let solicitacao = query.into_inner();
    repository.get_one(solicitacao).await
        .map(|result| HttpResponse::Ok().json(result))
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn get_all_solicitacao(app: Data<App>, query: Query<OptionSolicitacao>, opt: Query<QueryFilter>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.solicitacao;
    let solicitacao = query.into_inner();
    repository.get_all(solicitacao, opt.into_inner().into()).await
        .map(|result| HttpResponse::Ok().json(result))
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn create_solicitacao(app: Data<App>, solicitacao: Json<Solicitacao>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.solicitacao;
    repository.create(Box::new(solicitacao.into_inner())).await
        .map(|result| {
            if result.is_some() {HttpResponse::Created().json(&Some(result))} else {HttpResponse::Ok().body("")}
        })
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn update_solicitacao(app: Data<App>, solicitacao: Json<OptionSolicitacao>, ids: Path<(String,String)>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.solicitacao;
    let (aluno_id, professor_id) = ids.into_inner();
    let aluno_id = ObjectId::parse_str(&aluno_id)?;
    let professor_id = ObjectId::parse_str(&professor_id)?;
    repository.update_one(
        Box::new(solicitacao.into_inner()), aluno_id,professor_id
    ).await
        .map(|result| {
            if result.is_some() {HttpResponse::Ok().json(&Some(result))} else {HttpResponse::Ok().body("")}
        })
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn delete_solicitacao(app: Data<App>, ids: Path<(String,String)>) -> Result<impl Responder, AppError> {
    let repository = &app.repositories.solicitacao;
    let (aluno_id, professor_id) = ids.into_inner();
    let aluno_id = ObjectId::parse_str(&aluno_id)?;
    let professor_id = ObjectId::parse_str(&professor_id)?;
    repository.delete_one(
        aluno_id, professor_id
    ).await
        .map(|result| HttpResponse::Ok().json(result))
        .map_err(|err| HANDLER(Box::new(err)))
}