use actix_web::{web::{Json,Data,Path,Query}, Responder};
use mongodb::bson::oid::ObjectId;
use crate::{infrastructure::database::schemas::user_schema::{OptionUser, User}, errors::AppError, port::query_filter::QueryFilter};
use crate::di::d_injection::App;
use crate::routes::handler::HANDLER;

// gs:r
pub async fn get_gestor(app: Data<App>, query: Query<OptionUser>, id: Path<String>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.gestor;
    let mut user = query.into_inner();
    user.id = Some(ObjectId::parse_str(id.into_inner())?);
    controller.get_one(&mut(user)).await
        .map_err(|err| HANDLER(Box::new(err)))
}
// gs:r
pub async fn get_all_gestor(app: Data<App>, query: Query<OptionUser>, options: Query<QueryFilter>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.gestor;
    let options = options.into_inner();
    let mut user = query.into_inner();
    controller.get_all_gestor(&mut(user), options.into()).await
        .map_err(|err| HANDLER(Box::new(err)))
}
// gs:c
pub async fn create_gestor(app: Data<App>, user: Json<User>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.gestor;
    controller.create_gestor(Box::new(user.into_inner())).await
        .map_err(|err| HANDLER(Box::new(err)))
}
// gs:u
pub async fn update_gestor(app: Data<App>, user: Json<OptionUser>, id: Path<String>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.gestor;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller.update_gestor(
        Box::new(user.into_inner()), &id
    ).await
        .map_err(|err| HANDLER(Box::new(err)))
}
// gs:d
pub async fn delete_gestor(app: Data<App>, id: Path<String>) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.gestor;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller.delete_gestor(
        &id
    ).await
        .map_err(|err| HANDLER(Box::new(err)))
}