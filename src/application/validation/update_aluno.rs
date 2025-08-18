use std::f64::consts::E;

use mongodb::bson::DateTime;
use once_cell::sync::Lazy;

use crate::{application::models::aluno::AlunoUpdateModel, errors::{AppError, AppErrorType}, infrastructure::database::schemas::user_schema::OptionUserSchema};

pub struct UpdateAlunoValidation{}

use regex::Regex;
static RE_EMAIL: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap());
static RE_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Za-z\s]+$").unwrap());

impl UpdateAlunoValidation{
    pub fn validate(aluno: &mut AlunoUpdateModel) -> Result<(OptionUserSchema), AppError> {
        if let Some(name) = &aluno.name{
            if !RE_NAME.is_match(&name) {
                return Err(AppError::new(Some("Invalid name".to_string()),None,AppErrorType::ValidationError));
            }
        }
        if let Some(matricula) = &aluno.matricula{
            if matricula.is_empty() {
                return Err(AppError::new(Some("Matricula cannot be empty".to_string()),None,AppErrorType::ValidationError));
            }
            if !matricula.chars().all(char::is_numeric) {
                return Err(AppError::new(Some("Matricula must be numeric".to_string()),None,AppErrorType::ValidationError));
            }
        }
        if let Some(email) = &aluno.email{
            if !RE_EMAIL.is_match(&email) {
                return Err(AppError::new(Some("Invalid email".to_string()),None,AppErrorType::ValidationError));
            }
        }
        Ok(OptionUserSchema::from(aluno.clone()))
    }
}