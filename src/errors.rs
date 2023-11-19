use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;
use mongodb::bson::extjson::de::Error as BsonError;
use mongodb::error::Error as MongoDbError;

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    ValidationError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(message),
                ..
            } => message.clone(),
            AppError {
                message: None,
                error_type: AppErrorType::ValidationError,
                ..
            } => "The body content is invalid".to_string(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

impl From<BsonError> for AppError {
    fn from(error: BsonError) -> AppError {
        AppError {
            message: None, 
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError
        }
    }
}
impl From<MongoDbError> for AppError {
    fn from(error: MongoDbError) -> AppError {
        AppError {
            message: None, 
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError
        }
    }
}
impl From<Box<dyn std::error::Error>> for AppError {
    fn from(error: Box<dyn std::error::Error>) -> AppError {
        AppError {
            message: None, 
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::ValidationError => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}