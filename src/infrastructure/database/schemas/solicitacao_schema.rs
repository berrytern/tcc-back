use serde::{Deserialize, Serialize};
use mongodb::bson::{DateTime,oid::ObjectId};


// #[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SolicitacaoSchema {
    pub id_aluno: ObjectId,
    pub id_professor: ObjectId, 
    pub status: String,
    pub description: String,
    pub comment: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
//#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionSolicitacaoSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_aluno: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_professor: Option<ObjectId>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime>,
}