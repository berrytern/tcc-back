use serde::{Serialize, Deserialize};
use crate::infrastructure::database::schemas::{
    auth_schema::Auth,
    user_schema::OptionUser};
#[derive(Serialize,Deserialize,Clone)]
pub struct Login{
    pub login: String,
    pub password: String,
}

impl From<Auth> for Login {
    fn from(value: Auth) -> Self {
        Login {
            login: value.email,
            password: value.password,
        }
    }
}
impl Into<OptionUser> for Login {
    fn into(self) -> OptionUser {
        OptionUser {
            id: None,
            email: Some(self.login),
            user_type: None,
            name: None,
            matricula: None,
            created_at: None,
            updated_at: None,
        }
    }
}