use crate::errors::AppError;

pub(crate) const HANDLER: fn(Box<dyn std::error::Error>) -> AppError = |err| {
    AppError::from(err)    
};
