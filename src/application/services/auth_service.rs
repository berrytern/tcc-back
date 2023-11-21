use crate::application::models::access_token::AccessToken;
use crate::application::models::json_token::JsonToken;
use crate::application::models::login::Login;
use crate::infrastructure::database::schemas::user_schema::OptionUser;
use crate::{
    errors::AppError,
    infrastructure::repository::{auth_repository::AuthRepository,
                    user_repository::UserRepository},
};
use crate::application::utils::user_scopes::UserScope;
use crate::utils::default::MAX_EXP;
use jsonwebtoken::{decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use pwhash::bcrypt::{self, BcryptSetup, BcryptVariant};

#[derive(Clone)]
pub struct AuthService {
    repository: Box<AuthRepository>,
    user_repository: Box<UserRepository>,
}

impl AuthService {
    pub fn new(repository: Box<AuthRepository>, user_repository: Box<UserRepository>) -> Self {
        AuthService { 
            repository,
            user_repository,
        }
    }

    pub async fn login(&self, login: Login, secret: &str, salt: &str) -> Result<AccessToken, AppError> {
        let mut option: OptionUser = login.clone().into();
        if let Some(user) = self.user_repository.get_one(&mut(option)).await?{
            // hash compare
            if bcrypt::verify(login.password, &user.password){
                if let Some(id) = user.id {
                    let json_token = JsonToken {
                        sub: id.to_string(),
                        company: "uepb".to_string(),
                        scope: match user.user_type.as_str() {
                            "aluno" => UserScope::ALUNO.to_string(),
                            "gestor" => UserScope::GESTOR.to_string(),
                            "professor" => UserScope::PROFESSOR.to_string(),
                            _ => Err(AppError::new(Some("invalid user_type".to_string()),Some("invalid user_type value".to_string()),crate::errors::AppErrorType::ValidationError))?,
                        },
                        exp: get_current_timestamp() + MAX_EXP,
                    };
                    let access_token = &encode(&Header::default(), &json_token, &EncodingKey::from_secret(secret.as_bytes()))?;
                    return Ok(AccessToken{
                        access_token: access_token.to_string(),
                        refresh_token: bcrypt::hash_with(BcryptSetup{
                            salt:None,cost: Some(14),variant:None},
                            access_token)?, 
                    });
                }
            } else {
                return Err(AppError::new(Some("invalid password".to_string()),Some("provide another input value".to_string()),crate::errors::AppErrorType::ValidationError));
            }
        } else {
            return Err(AppError::new(Some("invalid user".to_string()),Some("invalid user".to_string()),crate::errors::AppErrorType::ValidationError));
        }
        Err(AppError::new(Some("".to_string()),Some("".to_string()),crate::errors::AppErrorType::ValidationError))
    }
}