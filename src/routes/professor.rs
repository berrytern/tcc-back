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

// pf:r
#[utoipa::path(responses((status = OK, body = UserOutput)))]
#[get("/v1/professores/{id}")]
pub async fn get_professor(
    app: Data<App>,
    query: Query<OptionUserSchema>,
    id: Path<String>,
) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.professor;
    let mut user = query.into_inner();
    user.id = Some(ObjectId::parse_str(id.into_inner())?.into());
    controller
        .get_one(&mut (user))
        .await
}
// pf:r
#[utoipa::path(responses((status = OK, body = Vec<UserOutput>)))]
#[get("/v1/professores")]
pub async fn get_all_professor(
    app: Data<App>,
    query: Query<OptionUserSchema>,
    options: Query<QueryFilter>,
) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.professor;
    let options = options.into_inner();
    let mut user = query.into_inner();
    controller
        .get_all_professor(&mut (user), options.into())
        .await
}

// pf:c
#[utoipa::path(responses((status = OK, body = UserOutput)))]
#[post("/v1/professores")]
pub async fn create_professor(
    app: Data<App>,
    user: Json<UserInput>,
) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.professor;
    controller
        .create_professor(user.into_inner())
        .await
}
// pf:u
#[utoipa::path(responses((status = OK, body = UserOutput)))]
#[patch("/v1/professores/{id}")]
pub async fn update_professor(
    app: Data<App>,
    user: Json<OptionUserSchema>,
    id: Path<String>,
) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.professor;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller
        .update_professor(Box::new(user.into_inner()), &id)
        .await
}
// pf:d
#[utoipa::path(responses((status = OK, body = bool)))]
#[delete("/v1/professores/{id}")]
pub async fn delete_professor(
    app: Data<App>,
    id: Path<String>,
) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.professor;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller
        .delete_professor(&id)
        .await
}
