use crate::{
    application::models::json_token::JsonToken,
    errors::{AppError,AppErrorType}
};


pub fn verify_access_by_scope(jwt_token: &JsonToken, scope: &str) -> Result<(), AppError> {
    if jwt_token.scope.contains(scope) {
        Ok(())
    } else {
        Err(AppError::new(Some("Unauthorized: Insufficient scope".to_string()), None, AppErrorType::Unauthorized))
    }
}