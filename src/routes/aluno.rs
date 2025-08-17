use crate::application::models::user::{UserInput, UserOutput};
use crate::di::d_injection::App;
use crate::{
    errors::AppError,
    infrastructure::database::schemas::user_schema::OptionUserSchema,
    port::query_filter::QueryFilter,
};
use actix_web::{delete, get, patch, post};
use actix_web::{
    web::{Data, Json, Path, Query},
    Responder,
};
use mongodb::bson::oid::ObjectId;

// al:r
#[utoipa::path(responses((status = OK, body = UserOutput)))]
#[get("/v1/alunos/{id}")]
pub async fn get_aluno(
    app: Data<App>,
    query: Query<OptionUserSchema>,
    id: Path<String>,
) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.aluno;
    let mut user = query.into_inner();
    user.id = Some(ObjectId::parse_str(id.into_inner())?.into());
    controller
        .get_one(&mut (user))
        .await
}
// al:r
#[utoipa::path(responses((status = OK, body = Vec<UserOutput>)))]
#[get("/v1/alunos")]
pub async fn get_all_aluno(
    app: Data<App>,
    query: Query<OptionUserSchema>,
    options: Query<QueryFilter>,
) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.aluno;
    let options = options.into_inner();
    let mut user = query.into_inner();
    controller
        .get_all_aluno(&mut (user), options.into())
        .await
}
// al:c
#[utoipa::path(responses((status = OK, body = UserOutput)))]
#[post("/v1/alunos")]
pub async fn create_aluno(app: Data<App>, user: Json<UserInput>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.aluno;
    controller
        .create_aluno(user.into_inner())
        .await
}
// al:u
#[utoipa::path(responses((status = OK, body = UserOutput)))]
#[patch("/v1/alunos/{id}")]
pub async fn update_aluno(
    app: Data<App>,
    user: Json<OptionUserSchema>,
    id: Path<String>,
) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.aluno;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller
        .update_aluno(Box::new(user.into_inner()), &id)
        .await
}
// al:d
#[utoipa::path(responses((status = OK, body = bool)))]
#[delete("/v1/alunos/{id}")]
pub async fn delete_aluno(app: Data<App>, id: Path<String>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.aluno;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller
        .delete_aluno(&id)
        .await
}
