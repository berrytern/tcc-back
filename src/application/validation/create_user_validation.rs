use mongodb::bson::DateTime;

use crate::{application::models::user::UserInput, errors::AppError, infrastructure::database::schemas::user_schema::UserSchema};

pub struct CreateUserValidation{}

impl CreateUserValidation{
    pub fn validate(user: &mut UserInput) -> Result<UserSchema,AppError> {
        let schema: UserSchema = user.into_schema(DateTime::now(), DateTime::now());
        Ok(schema)
    }
}