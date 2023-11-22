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

// pf:r
pub async fn get_professor(
    app: Data<App>,
    query: Query<OptionUserSchema>,
    id: Path<String>,
) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.professor;
    let mut user = query.into_inner();
    user.id = Some(ObjectId::parse_str(id.into_inner())?);
    controller
        .get_one(&mut (user))
        .await
        .map_err(|err| HANDLER(Box::new(err)))
}
// pf:r
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
        .map_err(|err| HANDLER(Box::new(err)))
}

// pf:c
pub async fn create_professor(
    app: Data<App>,
    user: Json<UserSchema>,
) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.professor;
    controller
        .create_professor(Box::new(user.into_inner()))
        .await
        .map_err(|err| HANDLER(Box::new(err)))
}
// pf:u
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
        .map_err(|err| HANDLER(Box::new(err)))
}
// pf:d
pub async fn delete_professor(
    app: Data<App>,
    id: Path<String>,
) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.professor;
    let id = ObjectId::parse_str(id.into_inner())?;
    controller
        .delete_professor(&id)
        .await
        .map_err(|err| HANDLER(Box::new(err)))
}
