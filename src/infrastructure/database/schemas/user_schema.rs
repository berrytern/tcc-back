use serde::{Deserialize, Serialize};

#[derive(strum_macros::Display)]
enum UserType{
    gestor,
    professor,
    aluno,
}
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
struct User {
    #[serde(alias = "_id")]
    pub id: Option<i32>,
    pub name: String,
    #[serde(alias = "type")]
    pub user_type: String,
    pub email: String, 
    pub matricula: Option<String>, 
}