use actix_web::{web::{Json,Data,Path,Query}, Responder};
use mongodb::bson::oid::ObjectId;
use crate::{infrastructure::database::schemas::user_schema::{OptionUserSchema, UserSchema}, errors::AppError, port::query_filter::QueryFilter};
use crate::di::d_injection::App;

// gs:r
pub async fn get_gestor(app: Data<App>, query: Query<OptionUserSchema>, id: Path<String>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.gestor;
    let mut user = query.into_inner();
    user.id = Some(ObjectId::parse_str(id.into_inner())?);
    controller.get_one(&mut(user)).await
}
// gs:r
pub async fn get_all_gestor(app: Data<App>, query: Query<OptionUserSchema>, options: Query<QueryFilter>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.gestor;
    let options = options.into_inner();
    let mut user = query.into_inner();
    controller.get_all_gestor(&mut(user), options.into()).await
}
// gs:c
pub async fn create_gestor(app: Data<App>, user: Json<UserSchema>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.gestor;
    controller.create_gestor(Box::new(user.into_inner())).await
}
// gs:u
pub async fn update_gestor(app: Data<App>, user: Json<OptionUserSchema>, id: Path<String>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.gestor;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller.update_gestor(
        Box::new(user.into_inner()), &id
    ).await
}
// gs:d
pub async fn delete_gestor(app: Data<App>, id: Path<String>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.gestor;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller.delete_gestor(
        &id
    ).await
}