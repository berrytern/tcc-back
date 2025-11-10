use crate::application::{middlewares::auth::verify_access_by_scope, models::{
    aluno::{AlunoQueryModel, AlunoUpdateModel, CreateAlunoModel}, json_token::JsonToken, user::{UserOutput}
}};
use crate::di::d_injection::App;
use crate::{
    errors::AppError,
    infrastructure::database::schemas::user_schema::OptionUserSchema,
    port::query_filter::QueryFilter,
};
use actix_web::{delete, get, http::header, patch, post, HttpResponse};
use actix_web::{
    web::{Data, Json, Path, Query},
    Responder,
};
use mongodb::bson::oid::ObjectId;

// al:r
#[utoipa::path(tag = "aluno", responses((status = OK, body = UserOutput)))]
#[get("/v1/alunos/{id}")]
pub async fn get_aluno(
    app: Data<App>,
    query: Query<AlunoQueryModel>,
    id: Path<String>,
    jwt_token: JsonToken
) -> Result<impl Responder, AppError> {
    verify_access_by_scope(&jwt_token, "al:r")?;
    let mut user: AlunoQueryModel = query.into_inner();
    user.id = Some(ObjectId::parse_str(id.into_inner())?.into());
    let mut user: OptionUserSchema = user.into();
    let controller = &app.controllers.aluno;
    controller
        .get_one(&mut (user))
        .await
}
// al:r
#[utoipa::path(tag = "aluno", responses((status = OK, body = Vec<UserOutput>)))]
#[get("/v1/alunos")]
pub async fn get_all_aluno(
    app: Data<App>,
    query: Query<AlunoQueryModel>,
    options: Query<QueryFilter>,
    jwt_token: JsonToken
) -> Result<impl Responder, AppError> {
    verify_access_by_scope(&jwt_token, "al:r")?;
    let user: AlunoQueryModel = query.into_inner();
    let mut user: OptionUserSchema = user.into();
    let redis = &mut app.redis_connection.clone();
    match redis.send_packed_command(redis::cmd("GET").arg(format!("/v1/alunos?{user}"))).await {
        Ok(redis::Value::BulkString(string))=>{
            Ok(HttpResponse::Ok().append_header(header::ContentType::json()).body(string))
        }
        _ => {
        let controller = &app.controllers.aluno;
        let options = options.into_inner();
        controller
            .get_all_aluno(&mut (user), options.into(), Some(redis.to_owned()))
            .await
        }
    }
}
// al:c
#[utoipa::path(tag = "aluno", responses((status = OK, body = UserOutput)))]
#[post("/v1/alunos")]
pub async fn create_aluno(app: Data<App>, user: Json<CreateAlunoModel>, jwt_token: JsonToken) -> Result<impl Responder, AppError> {
    println!("here");
    verify_access_by_scope(&jwt_token, "al:c")?;
    println!("here2");
    let controller = &app.controllers.aluno;
    controller
        .create_aluno(user.into_inner())
        .await
}
// al:u
#[utoipa::path(tag = "aluno", responses((status = OK, body = UserOutput)))]
#[patch("/v1/alunos/{id}")]
pub async fn update_aluno(
    app: Data<App>,
    user: Json<AlunoUpdateModel>,
    id: Path<String>,
    jwt_token: JsonToken
) -> Result<impl Responder, AppError> {
    verify_access_by_scope(&jwt_token, "al:u")?;
    let controller = &app.controllers.aluno;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller
        .update_aluno(user.into_inner(), &id)
        .await
}
// al:d
#[utoipa::path(tag = "aluno", responses((status = OK, body = bool)))]
#[delete("/v1/alunos/{id}")]
pub async fn delete_aluno(app: Data<App>, id: Path<String>, jwt_token: JsonToken) -> Result<impl Responder, AppError> {
    verify_access_by_scope(&jwt_token, "al:d")?;
    let controller = &app.controllers.aluno;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller
        .delete_aluno(&id)
        .await
}
