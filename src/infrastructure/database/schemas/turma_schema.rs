use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::infrastructure::database::schemas::user_schema::{MyObjectId, MyDateTime};

// #[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct Turma {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MyObjectId>,
    pub id_aluno: MyObjectId,
    pub id_professor: MyObjectId,
    pub active: bool,
    pub created_at: Option<MyDateTime>,
    pub updated_at: Option<MyDateTime>,
}
//#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
pub struct OptionTurma {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MyObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_aluno: Option<MyObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_professor: Option<MyObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<MyDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<MyDateTime>,
}