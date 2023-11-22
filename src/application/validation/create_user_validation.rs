use mongodb::bson::DateTime;

use crate::{errors::AppError, infrastructure::database::schemas::user_schema::UserSchema};

pub struct CreateUserValidation{}

impl CreateUserValidation{
    pub fn validate(user: &mut UserSchema) -> Result<(),AppError> {
        user.id = None;
        user.created_at = Some(DateTime::now());
        user.updated_at = Some(DateTime::now());
        Ok(())
    }
}