use mongodb::bson::DateTime;

use crate::{errors::AppError, infrastructure::database::schemas::user_schema::User};

pub struct CreateUserValidation{}

impl CreateUserValidation{
    pub fn validate(user: &mut User) -> Result<(),AppError> {
        user.id = None;
        user.created_at = Some(DateTime::now());
        user.updated_at = Some(DateTime::now());
        Ok(())
    }
}