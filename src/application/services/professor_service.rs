use mongodb::bson::oid::ObjectId;
use pwhash::bcrypt;
use crate::application::models::user::User;
use crate::application::validation::create_user_validation::CreateUserValidation;
use crate::application::validation::update_user_validation::UpdateUserValidation;
use crate::{infrastructure::{repository::professor_repository::ProfessorRepository, database::schemas::user_schema::{UserSchema, OptionUserSchema}}, errors::AppError, port::query_filter::QueryOptions};

#[derive(Clone)]
pub struct ProfessorService{
    repository: Box<ProfessorRepository>
}

impl ProfessorService {
    pub fn new(repository: Box<ProfessorRepository>) -> Self{
        ProfessorService{
            repository
        }
    }

    pub async fn get_one(&self, user: &mut OptionUserSchema) -> Result<Option<User>, AppError> {
        Ok(self.repository.get_one(user).await.map(|op| op.map(|item |User::from(item)))?)
    }
    pub async fn get_all_professor(&self, user: &mut OptionUserSchema, options: QueryOptions) -> Result<Vec<User>, AppError> {
        Ok(self
            .repository
            .get_all(user, options).await.map( |item| item.into_iter().map(|f| User::from(f)).collect::<Vec<User>>())?)
    }
    
    pub async fn create_professor(&self, mut user: Box<UserSchema>) -> Result<Option<User>, AppError> {
        CreateUserValidation::validate(&mut(user))?;
        user.password = bcrypt::hash(user.password)?;
        user.user_type = "professor".to_string();
        Ok(self.repository.create(user).await
            .map(|opt_user| opt_user.map(|user| User::from(*user)))?)
    }
    
    pub async fn update_professor(&self, mut user: Box<OptionUserSchema>, id: &ObjectId) -> Result<Option<User>, AppError> {
        UpdateUserValidation::validate(&mut(user))?;
        Ok(self.repository.update_one(
            user, id
        ).await.map(|op|op.map(|item|User::from(item)))?)
    }
    
    pub async fn delete_professor(&self, id: &ObjectId) -> Result<bool, AppError> {
        Ok(self.repository.delete_one(
            id
        ).await?)
    }

}