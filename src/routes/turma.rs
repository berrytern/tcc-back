use actix_web::{web::{Json,Data,Path,Query}, Responder};
use mongodb::bson::oid::ObjectId;
use crate::{infrastructure::database::schemas::turma_schema::{OptionTurma, Turma}, errors::AppError, port::query_filter::QueryFilter};
use crate::di::d_injection::App;
use crate::routes::handler::HANDLER;


pub async fn get_one_turma(app: Data<App>, query: Query<OptionTurma>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.turma;
    let turma = query.into_inner();
    controller.get_one(&turma).await
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn get_all_turma(app: Data<App>, query: Query<OptionTurma>, opt: Query<QueryFilter>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.turma;
    let turma = query.into_inner();
    controller.get_all_turma(&turma, opt.into_inner().into()).await
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn create_turma(app: Data<App>, turma: Json<Turma>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.turma;
    controller.create_turma(Box::new(turma.into_inner())).await
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn update_turma(app: Data<App>, turma: Json<OptionTurma>, ids: Path<(String,String)>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.turma;
    let (aluno_id, professor_id) = ids.into_inner();
    let aluno_id = ObjectId::parse_str(&aluno_id)?;
    let professor_id = ObjectId::parse_str(&professor_id)?;
    controller.update_turma(
        Box::new(turma.into_inner()), &aluno_id,&professor_id
    ).await
        .map_err(|err| HANDLER(Box::new(err)))
}

pub async fn delete_turma(app: Data<App>, ids: Path<(String,String)>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.turma;
    let (aluno_id, professor_id) = ids.into_inner();
    let aluno_id = ObjectId::parse_str(&aluno_id)?;
    let professor_id = ObjectId::parse_str(&professor_id)?;
    controller.delete_turma(
        &aluno_id, &professor_id
    ).await
        .map_err(|err| HANDLER(Box::new(err)))
}