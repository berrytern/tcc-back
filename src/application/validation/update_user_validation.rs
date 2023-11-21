use mongodb::bson::DateTime;

use crate::{errors::AppError, infrastructure::database::schemas::user_schema::OptionUser};

pub struct UpdateUserValidation{}

impl UpdateUserValidation{
    pub fn validate(user: &mut OptionUser) -> Result<(),AppError> {
        user.id = None;
        user.user_type = None;
        user.created_at = None;
        user.updated_at = Some(DateTime::now());
        Ok(())
    }
}