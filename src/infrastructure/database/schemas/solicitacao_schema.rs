use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::infrastructure::database::schemas::user_schema::{MyDateTime, MyObjectId};


// #[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct SolicitacaoSchema {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MyObjectId>,
    pub id_aluno: MyObjectId,
    pub id_professor: MyObjectId, 
    pub status: String,
    pub description: String,
    pub comment: String,
    pub created_at: Option<MyDateTime>,
    pub updated_at: Option<MyDateTime>,
}
//#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct OptionSolicitacaoSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_aluno: Option<MyObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_professor: Option<MyObjectId>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<MyDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<MyDateTime>,
}