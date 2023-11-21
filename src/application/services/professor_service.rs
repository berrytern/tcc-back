use mongodb::bson::oid::ObjectId;
use crate::application::validation::create_user_validation::CreateUserValidation;
use crate::application::validation::update_user_validation::UpdateUserValidation;
use crate::{infrastructure::{repository::professor_repository::ProfessorRepository, database::schemas::user_schema::{User, OptionUser}}, errors::AppError, port::query_filter::QueryOptions};

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

    pub async fn get_one(&self, user: &OptionUser) -> Result<Option<User>, AppError> {
        Ok(self.repository.get_one(user).await?)
    }
    pub async fn get_all_professor(&self, user: OptionUser, options: QueryOptions) -> Result<Vec<User>, AppError> {
        Ok(self.repository.get_all(user, options).await?)
    }
    
    pub async fn create_professor(&self, mut user: Box<User>) -> Result<Option<Box<User>>, AppError> {
        CreateUserValidation::validate(&mut(*user))?;
        user.user_type = "professor".to_string();
        Ok(self.repository.create(user).await?)
    }
    
    pub async fn update_professor(&self, mut user: Box<OptionUser>, id: &ObjectId) -> Result<Option<User>, AppError> {
        UpdateUserValidation::validate(&mut(*user))?;
        Ok(self.repository.update_one(
            user, id
        ).await?)
    }
    
    pub async fn delete_professor(&self, id: &ObjectId) -> Result<bool, AppError> {
        Ok(self.repository.delete_one(
            id
        ).await?)
    }

}