use crate::di::d_injection::App;
use crate::routes::handler::HANDLER;
use crate::{
    errors::AppError,
    infrastructure::database::schemas::user_schema::{OptionUserSchema, UserSchema},
    port::query_filter::QueryFilter,
};
use actix_web::{
    web::{Data, Json, Path, Query},
    Responder,
};
use mongodb::bson::oid::ObjectId;

// al:r
pub async fn get_aluno(
    app: Data<App>,
    query: Query<OptionUserSchema>,
    id: Path<String>,
) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.aluno;
    let mut user = query.into_inner();
    user.id = Some(ObjectId::parse_str(id.into_inner())?);
    controller
        .get_one(&mut (user))
        .await
        .map_err(|err| HANDLER(Box::new(err)))
}
// al:r
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
        .map_err(|err| HANDLER(Box::new(err)))
}
// al:c
pub async fn create_aluno(app: Data<App>, user: Json<UserSchema>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.aluno;
    controller
        .create_aluno(Box::new(user.into_inner()))
        .await
        .map_err(|err| HANDLER(Box::new(err)))
}
// al:u
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
        .map_err(|err| HANDLER(Box::new(err)))
}
// al:d
pub async fn delete_aluno(app: Data<App>, id: Path<String>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.aluno;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller
        .delete_aluno(&id)
        .await
        .map_err(|err| HANDLER(Box::new(err)))
}
