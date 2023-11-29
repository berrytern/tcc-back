use crate::di::d_injection::App;
use crate::routes::handler::HANDLER;
use crate::{
    errors::AppError,
    infrastructure::database::schemas::user_schema::{OptionUser, User},
    port::query_filter::QueryFilter,
};
use actix_web::{
    web::{Data, Json, Path, Query},
    Responder,
};
use mongodb::bson::oid::ObjectId;

pub async fn login(
    app: Data<App>,
    query: Query<OptionUser>,
    id: Path<String>,
) -> Result<impl Responder, AppError> {
    let controller = &app.controllers.aluno;
    let mut user = query.into_inner();
    user.id = Some(ObjectId::parse_str(id.into_inner())?);
    controller
        .get_one(&user)
        .await
}
