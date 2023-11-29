use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt::{self, Display};
use mongodb::bson::extjson::de::Error as BsonError;
use mongodb::bson::oid::Error as OidError;
use mongodb::error::Error as MongoDbError;
use mongodb::error::{WriteFailure,WriteConcernError,WriteError};

#[derive(Debug)]
pub enum AppErrorType {
    InternalError,
    ValidationError,
    ConflictError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub description: Option<String>,
    pub error_type: AppErrorType,
}
impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r"\{{ {:?}, {:?})\}}", self.message, self.description)
    }
}
impl AppError {
    pub fn new(description: Option<String>, message: Option<String>, error_type: AppErrorType) -> AppError {
        AppError {
            description,
            message,
            error_type,
        }
    }
    pub fn get_message(&self) -> String {
        match self {
            AppError {
                error_type: AppErrorType::ValidationError,
                ..
            } => "The body content is invalid".to_string(),
            AppError {
                error_type: AppErrorType::ConflictError,
                ..
            } => "Unique field value already in used".to_string(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(error: Box<dyn std::error::Error>) -> AppError {
        println!("{:?}", error);
        AppError {
            message: None, 
            description: Some(error.to_string()),
            error_type: AppErrorType::ValidationError
        }
    }
}
impl From<pwhash::error::Error> for AppError {
    fn from(error: pwhash::error::Error) -> Self {
        AppError {
            message: None, 
            description: Some(error.to_string()),
            error_type: AppErrorType::ValidationError
        }
    }
}
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(error: jsonwebtoken::errors::Error) -> AppError {
        AppError {
            message: None, 
            description: Some(error.to_string()),
            error_type: AppErrorType::ValidationError
        }
    }
}
impl From<BsonError> for AppError {
    fn from(error: BsonError) -> AppError {
        AppError {
            message: None, 
            description: Some(error.to_string()),
            error_type: AppErrorType::ValidationError
        }
    }
}
impl From<WriteFailure> for AppError {
    fn from(_error: WriteFailure) -> AppError {
        AppError {
            message: None, 
            description: None,
            error_type: AppErrorType::ConflictError
        }
    }
}
impl From<WriteConcernError> for AppError {
    fn from(_error: WriteConcernError) -> AppError {
        AppError {
            message: None, 
            description: None,
            error_type: AppErrorType::ConflictError
        }
    }
}
impl From<WriteError> for AppError {
    fn from(_error: WriteError) -> AppError {
        AppError {
            message: None, 
            description: None,
            error_type: AppErrorType::ConflictError
        }
    }
}
impl From<OidError> for AppError {
    fn from(error: OidError) -> AppError {
        AppError {
            message: None, 
            description: Some(error.to_string()),
            error_type: AppErrorType::ValidationError
        }
    }
}
impl From<MongoDbError> for AppError {
    fn from(error: MongoDbError) -> AppError {
        AppError {
            message: None, 
            description: Some(error.to_string()),
            error_type: AppErrorType::ValidationError,
        }
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub cause: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub description: String
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::ValidationError => StatusCode::BAD_REQUEST,
            AppErrorType::ConflictError => StatusCode::CONFLICT,
        }
    }
    fn error_response(&self) -> HttpResponse {
        if let Some(description) = self.description.clone() {
            HttpResponse::build(self.status_code()).json(AppErrorResponse {
                cause: self.get_message(),
                message: self.message.clone(),
                description: description,
            })
        } else {
            HttpResponse::build(self.status_code()).json(AppErrorResponse {
                cause: self.get_message(),
                message: None,
                description: "".to_string(),
            })
        }
    }
}