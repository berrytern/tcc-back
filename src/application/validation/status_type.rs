use crate::errors::{AppError,AppErrorType};

pub struct StatusType {}
impl StatusType {
    pub fn validate(value: &str) -> Result<&str, AppError> {
        match value {
            "pending" => Ok("pending"),
            "accepted" => Ok("accepted"),
            _ => Err(AppError{
                message: Some("invalid status field".to_string()),
                description: Some(format!("invalid status field: the value '{value}' must be one of [pending,accepted]")),
                error_type: AppErrorType::ValidationError
            })
        }
    }
}