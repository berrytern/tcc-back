use actix_web::{delete, get, patch, post, web::{Data, Json, Path, Query}, Responder};
use mongodb::bson::oid::ObjectId;
use crate::{application::{middlewares::auth::verify_access_by_scope, models::{json_token::JsonToken, user::{UserInput, UserOutput}}}, errors::AppError, infrastructure::database::schemas::user_schema::OptionUserSchema, port::query_filter::QueryFilter};
use crate::di::d_injection::App;

// gs:r
#[utoipa::path(tag = "gestor", responses((status = OK, body = Option<UserOutput>)))]
#[get("/v1/gestores/{id}")]
pub async fn get_gestor(app: Data<App>, query: Query<OptionUserSchema>, id: Path<String>, jwt_token: JsonToken) -> Result<impl Responder, AppError> {
    verify_access_by_scope(&jwt_token, "gs:r")?;
    let controller = &app.controllers.gestor;
    let mut user = query.into_inner();
    user.id = Some(ObjectId::parse_str(id.into_inner())?.into());
    controller.get_one(&mut(user)).await
}
// gs:r
#[utoipa::path(tag = "gestor", responses((status = OK, body = Vec<UserOutput>)))]
#[get("/v1/gestores")]
pub async fn get_all_gestor(app: Data<App>, query: Query<OptionUserSchema>, options: Query<QueryFilter>, jwt_token: JsonToken) -> Result<impl Responder, AppError> {
    verify_access_by_scope(&jwt_token, "gs:r")?;
    let controller = &app.controllers.gestor;
    let options = options.into_inner();
    let mut user = query.into_inner();
    controller.get_all_gestor(&mut(user), options.into()).await
}
// gs:c
#[utoipa::path(tag = "gestor", responses((status = OK, body = UserOutput)))]
#[post("/v1/gestores")]
pub async fn create_gestor(app: Data<App>, user: Json<UserInput>) -> Result<impl Responder, AppError> {
    // verify_access_by_scope(&jwt_token, "gs:c")?;
    let controller = &app.controllers.gestor;
    controller.create_gestor(user.into_inner()).await
}
// gs:u
#[utoipa::path(tag = "gestor", responses((status = OK, body = UserOutput)))]
#[patch("/v1/gestores/{id}")]
pub async fn update_gestor(app: Data<App>, user: Json<OptionUserSchema>, id: Path<String>, jwt_token: JsonToken) -> Result<impl Responder, AppError> {
    verify_access_by_scope(&jwt_token, "gs:u")?;
    let controller = &app.controllers.gestor;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller.update_gestor(
        Box::new(user.into_inner()), &id
    ).await
}
// gs:d
#[utoipa::path(tag = "gestor", responses((status = OK, body = bool)))]
#[delete("/v1/gestores/{id}")]
pub async fn delete_gestor(app: Data<App>, id: Path<String>, jwt_token: JsonToken) -> Result<impl Responder, AppError> {
    verify_access_by_scope(&jwt_token, "gs:d")?;
    let controller = &app.controllers.gestor;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller.delete_gestor(
        &id
    ).await
}