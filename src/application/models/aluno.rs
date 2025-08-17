use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Clone)]
#[derive(utoipa::ToSchema)]
pub struct AlunoUpdate{
    pub name: Option<String>,
    pub email: Option<String>,
    pub matricula: Option<String>,
}