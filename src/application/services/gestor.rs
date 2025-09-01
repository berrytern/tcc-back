use mongodb::bson::oid::ObjectId;
use pwhash::bcrypt;
use crate::application::models::user::{UserInput, UserOutput};
use crate::application::validation::create_user::CreateUserValidation;
use crate::application::validation::update_user::UpdateUserValidation;
use crate::{infrastructure::{repository::gestor_repository::GestorRepository, database::schemas::user_schema::OptionUserSchema}, errors::AppError, port::query_filter::QueryOptions};

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

    pub async fn get_one(&self, user: &mut OptionUserSchema) -> Result<Option<UserOutput>, AppError> {
        Ok(self.repository.get_one(user).await.map(|op| op.map(UserOutput::from))?)
    }
    pub async fn get_all_gestor(&self, user: &mut OptionUserSchema, options: QueryOptions) -> Result<Vec<UserOutput>, AppError> {
        Ok(self
            .repository
            .get_all(user, options).await.map( |item| item.into_iter().map(UserOutput::from).collect::<Vec<UserOutput>>())?)
    }
    
    pub async fn create_gestor(&self, user: UserInput) -> Result<Option<UserOutput>, AppError> {
        let mut user = CreateUserValidation::validate(user, "gestor")?;
        user.password = bcrypt::hash(user.password)?;
        user.matricula = None;
        Ok(self.repository.create(user).await
            .map(|opt_user| opt_user.map(UserOutput::from))?)
    }
    
    pub async fn update_gestor(&self, mut user: Box<OptionUserSchema>, id: &ObjectId) -> Result<Option<UserOutput>, AppError> {
        UpdateUserValidation::validate(&mut(user))?;
        user.matricula = None;
        Ok(self.repository.update_one(
            user, id
        ).await.map(|op|op.map(UserOutput::from))?)
    }
    
    pub async fn delete_gestor(&self, id: &ObjectId) -> Result<bool, AppError> {
        Ok(self.repository.delete_one(
            id
        ).await?)
    }

}