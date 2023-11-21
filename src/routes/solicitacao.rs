use actix_web::{web::{Json,Data,Path,Query}, Responder};
use mongodb::bson::oid::ObjectId;
use crate::{infrastructure::database::schemas::solicitacao_schema::{OptionSolicitacao, Solicitacao}, errors::AppError, port::query_filter::QueryFilter};
use crate::di::d_injection::App;
use crate::routes::handler::HANDLER;


pub async fn get_one_solicitacao(app: Data<App>, query: Query<OptionSolicitacao>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.solicitacao;
    let solicitacao = query.into_inner();
    controller.get_one(&solicitacao).await
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn get_all_solicitacao(app: Data<App>, query: Query<OptionSolicitacao>, opt: Query<QueryFilter>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.solicitacao;
    let solicitacao = query.into_inner();
    controller.get_all_solicitacao(&solicitacao, opt.into_inner().into()).await
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn create_solicitacao(app: Data<App>, solicitacao: Json<Solicitacao>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.solicitacao;
    controller.create_solicitacao(Box::new(solicitacao.into_inner())).await
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn update_solicitacao(app: Data<App>, solicitacao: Json<OptionSolicitacao>, ids: Path<(String,String)>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.solicitacao;
    let (aluno_id, professor_id) = ids.into_inner();
    let aluno_id = ObjectId::parse_str(&aluno_id)?;
    let professor_id = ObjectId::parse_str(&professor_id)?;
    controller.update_solicitacao(
        Box::new(solicitacao.into_inner()), &aluno_id,&professor_id
    ).await
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn delete_solicitacao(app: Data<App>, ids: Path<(String,String)>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.solicitacao;
    let (aluno_id, professor_id) = ids.into_inner();
    let aluno_id = ObjectId::parse_str(&aluno_id)?;
    let professor_id = ObjectId::parse_str(&professor_id)?;
    controller.delete_solicitacao(
        &aluno_id, &professor_id
    ).await
        .map_err(|err| HANDLER(Box::new(err)))
}