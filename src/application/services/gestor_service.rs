use mongodb::bson::oid::ObjectId;
use crate::application::validation::create_user_validation::CreateUserValidation;
use crate::application::validation::update_user_validation::UpdateUserValidation;
use crate::{infrastructure::{repository::gestor_repository::GestorRepository, database::schemas::user_schema::{User, OptionUser}}, errors::AppError, port::query_filter::QueryOptions};

#[derive(Clone)]
pub struct GestorService{
    repository: Box<GestorRepository>
}

impl GestorService {
    pub fn new(repository: Box<GestorRepository>) -> Self{
        GestorService{
            repository
        }
    }

    pub async fn get_one(&self, user: &OptionUser) -> Result<Option<User>, AppError> {
        Ok(self.repository.get_one(user).await?)
    }
    pub async fn get_all_gestor(&self, user: OptionUser, options: QueryOptions) -> Result<Vec<User>, AppError> {
        Ok(self.repository.get_all(user, options).await?)
    }
    
    pub async fn create_gestor(&self, mut user: Box<User>) -> Result<Option<Box<User>>, AppError> {
        CreateUserValidation::validate(&mut(*user))?;
        user.user_type = "gestor".to_string();
        user.matricula = None;
        Ok(self.repository.create(user).await?)
    }
    
    pub async fn update_gestor(&self, mut user: Box<OptionUser>, id: &ObjectId) -> Result<Option<User>, AppError> {
        UpdateUserValidation::validate(&mut(*user))?;
        user.matricula = None;
        Ok(self.repository.update_one(
            user, id
        ).await?)
    }
    
    pub async fn delete_gestor(&self, id: &ObjectId) -> Result<bool, AppError> {
        Ok(self.repository.delete_one(
            id
        ).await?)
    }

}