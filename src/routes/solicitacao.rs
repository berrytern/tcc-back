use actix_web::{delete, get, patch, post, web::{Data, Json, Path, Query}, Responder};
use mongodb::bson::oid::ObjectId;
use crate::{application::models::solicitacao::{CreateSolicitacaoModel, UpdateSolicitacaoModel}, errors::AppError, infrastructure::database::schemas::solicitacao_schema::{OptionSolicitacaoSchema, SolicitacaoSchema}, port::query_filter::QueryFilter};
use crate::di::d_injection::App;

// pf:r
#[utoipa::path(tag = "solicitacao", responses((status = OK, body = OptionSolicitacaoSchema)))]
#[get("/v1/solicitacao")]
pub async fn get_one_solicitacao(app: Data<App>, query: Query<OptionSolicitacaoSchema>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.solicitacao;
    let solicitacao = query.into_inner();
    controller.get_one(&solicitacao).await
}
// pf:r
#[utoipa::path(tag = "solicitacao", responses((status = OK, body = Vec<SolicitacaoSchema>)))]
#[get("/v1/solicitacoes")]
pub async fn get_all_solicitacao(app: Data<App>, query: Query<OptionSolicitacaoSchema>, opt: Query<QueryFilter>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.solicitacao;
    let solicitacao = query.into_inner();
    controller.get_all_solicitacao(&solicitacao, opt.into_inner().into()).await
}
// pf:c
#[utoipa::path(tag = "solicitacao", responses((status = OK, body = SolicitacaoSchema)))]
#[post("/v1/solicitacoes")]
pub async fn create_solicitacao(app: Data<App>, solicitacao: Json<CreateSolicitacaoModel>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.solicitacao;
    controller.create_solicitacao(solicitacao.into_inner().into()).await
}
// pf:u
#[utoipa::path(tag = "solicitacao", responses((status = OK, body = SolicitacaoSchema)))]
#[patch("/v1/solicitacoes/{aluno_id}/{professor_id}")]
pub async fn update_solicitacao(app: Data<App>, solicitacao: Json<UpdateSolicitacaoModel>, ids: Path<(String,String)>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.solicitacao;
    let (aluno_id, professor_id) = ids.into_inner();
    let aluno_id = ObjectId::parse_str(&aluno_id)?;
    let professor_id = ObjectId::parse_str(&professor_id)?;
    controller.update_solicitacao(
        solicitacao.into_inner(), &aluno_id,&professor_id
    ).await
}
// pf:d
#[utoipa::path(tag = "solicitacao", responses((status = OK, body = bool)))]
#[delete("/v1/solicitacoes/{aluno_id}/{professor_id}")]
pub async fn delete_solicitacao(app: Data<App>, ids: Path<(String,String)>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.solicitacao;
    let (aluno_id, professor_id) = ids.into_inner();
    let aluno_id = ObjectId::parse_str(&aluno_id)?;
    let professor_id = ObjectId::parse_str(&professor_id)?;
    controller.delete_solicitacao(
        &aluno_id, &professor_id
    ).await
}