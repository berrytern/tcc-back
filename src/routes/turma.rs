use actix_web::{delete, get, patch, post, web::{Data, Json, Path, Query}, Responder};
use mongodb::bson::oid::ObjectId;
use crate::{infrastructure::database::schemas::turma_schema::{OptionTurma, Turma}, errors::AppError, port::query_filter::QueryFilter};
use crate::di::d_injection::App;

// tr:r
#[utoipa::path(tag = "turma", responses((status = OK, body = Turma)))]
#[get("/v1/turma")]
pub async fn get_one_turma(app: Data<App>, query: Query<OptionTurma>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.turma;
    let turma = query.into_inner();
    controller.get_one(&turma).await
}
// tr:r
#[utoipa::path(tag = "turma", responses((status = OK, body = Vec<Turma>)))]
#[get("/v1/turmas")]
pub async fn get_all_turma(app: Data<App>, query: Query<OptionTurma>, opt: Query<QueryFilter>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.turma;
    let turma = query.into_inner();
    controller.get_all_turma(&turma, opt.into_inner().into()).await
}
// tr:c
#[utoipa::path(tag = "turma", responses((status = OK, body = Turma)))]
#[post("/v1/turmas")]
pub async fn create_turma(app: Data<App>, turma: Json<Turma>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.turma;
    controller.create_turma(Box::new(turma.into_inner())).await
}
// tr:u
#[utoipa::path(tag = "turma", responses((status = OK, body = Turma)))]
#[patch("/v1/turmas/{aluno_id}/{professor_id}")]
pub async fn update_turma(app: Data<App>, turma: Json<OptionTurma>, ids: Path<(String,String)>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.turma;
    let (aluno_id, professor_id) = ids.into_inner();
    let aluno_id = ObjectId::parse_str(&aluno_id)?;
    let professor_id = ObjectId::parse_str(&professor_id)?;
    controller.update_turma(
        Box::new(turma.into_inner()), &aluno_id,&professor_id
    ).await
}
// tr:d
#[utoipa::path(tag = "turma", responses((status = OK, body = bool)))]
#[delete("/v1/turmas/{aluno_id}/{professor_id}")]
pub async fn delete_turma(app: Data<App>, ids: Path<(String,String)>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.turma;
    let (aluno_id, professor_id) = ids.into_inner();
    let aluno_id = ObjectId::parse_str(&aluno_id)?;
    let professor_id = ObjectId::parse_str(&professor_id)?;
    controller.delete_turma(
        &aluno_id, &professor_id
    ).await
}