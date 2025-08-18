use mongodb::bson::DateTime;
use serde::{Serialize,Deserialize};

use crate::infrastructure::database::schemas::user_schema::OptionUserSchema;

#[derive(Serialize,Deserialize,Clone)]
#[derive(utoipa::ToSchema)]
pub struct AlunoUpdateModel{
    pub name: Option<String>,
    pub email: Option<String>,
    pub matricula: Option<String>,
}
impl From<OptionUserSchema> for AlunoUpdateModel {
    fn from(user: OptionUserSchema) -> Self {
        AlunoUpdateModel {
            name: user.name,
            email: user.email,
            matricula: user.matricula,
        }
    }
}
impl From<AlunoUpdateModel> for OptionUserSchema {
    fn from(user: AlunoUpdateModel) -> Self {
        OptionUserSchema {
            id: None,
            name: user.name,
            email: user.email,
            matricula: user.matricula,
            user_type: None,
            created_at: None,
            updated_at: Some(DateTime::now().into()),
        }
    }
}