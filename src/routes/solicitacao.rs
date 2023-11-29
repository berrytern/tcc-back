use actix_web::{web::{Json,Data,Path,Query}, Responder};
use mongodb::bson::oid::ObjectId;
use crate::{infrastructure::database::schemas::solicitacao_schema::{OptionSolicitacaoSchema, SolicitacaoSchema}, errors::AppError, port::query_filter::QueryFilter};
use crate::di::d_injection::App;

// pf:r
pub async fn get_one_solicitacao(app: Data<App>, query: Query<OptionSolicitacaoSchema>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.solicitacao;
    let solicitacao = query.into_inner();
    controller.get_one(&solicitacao).await
}
// pf:r
pub async fn get_all_solicitacao(app: Data<App>, query: Query<OptionSolicitacaoSchema>, opt: Query<QueryFilter>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.solicitacao;
    let solicitacao = query.into_inner();
    controller.get_all_solicitacao(&solicitacao, opt.into_inner().into()).await
}
// pf:c
pub async fn create_solicitacao(app: Data<App>, solicitacao: Json<SolicitacaoSchema>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.solicitacao;
    controller.create_solicitacao(Box::new(solicitacao.into_inner())).await
}
// pf:u
pub async fn update_solicitacao(app: Data<App>, solicitacao: Json<OptionSolicitacaoSchema>, ids: Path<(String,String)>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.solicitacao;
    let (aluno_id, professor_id) = ids.into_inner();
    let aluno_id = ObjectId::parse_str(&aluno_id)?;
    let professor_id = ObjectId::parse_str(&professor_id)?;
    controller.update_solicitacao(
        Box::new(solicitacao.into_inner()), &aluno_id,&professor_id
    ).await
}
// pf:d
pub async fn delete_solicitacao(app: Data<App>, ids: Path<(String,String)>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.solicitacao;
    let (aluno_id, professor_id) = ids.into_inner();
    let aluno_id = ObjectId::parse_str(&aluno_id)?;
    let professor_id = ObjectId::parse_str(&professor_id)?;
    controller.delete_solicitacao(
        &aluno_id, &professor_id
    ).await
}