use serde::{Serialize, Deserialize};
use crate::infrastructure::database::schemas::user_schema::OptionUserSchema;
#[derive(Serialize,Deserialize,Clone)]
pub struct Login{
    pub login: String,
    pub password: String,
}

impl From<Login> for OptionUserSchema {
    fn from(val: Login) -> Self {
        OptionUserSchema {
            id: None,
            email: Some(val.login),
            user_type: None,
            name: None,
            matricula: None,
            created_at: None,
            updated_at: None,
            }
        }
}