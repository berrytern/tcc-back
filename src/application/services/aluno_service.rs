use crate::application::models::user::{UserInput, UserOutput};
use crate::application::validation::{
    create_user_validation::CreateUserValidation, update_user_validation::UpdateUserValidation,
};
use crate::{
    errors::AppError,
    infrastructure::{
        database::schemas::user_schema::OptionUserSchema,
        repository::aluno_repository::AlunoRepository,
    },
    port::query_filter::QueryOptions,
};
use mongodb::bson::oid::ObjectId;
use pwhash::bcrypt;

#[derive(Clone)]
pub struct AlunoService {
    repository: Box<AlunoRepository>,
}

impl AlunoService {
    pub fn new(repository: Box<AlunoRepository>) -> Self {
        AlunoService { repository }
    }

    pub async fn get_one(&self, user: &mut OptionUserSchema) -> Result<Option<UserOutput>, AppError> {
        Ok(self.repository.get_one(user).await.map(|op| op.map(|item |UserOutput::from(item)))?)
    }
    pub async fn get_all_aluno(
        &self,
        user: &mut OptionUserSchema,
        options: QueryOptions,
    ) -> Result<Vec<UserOutput>, AppError> {
        Ok(self
            .repository
            .get_all(user, options).await.map( |item| item.into_iter().map(|f| UserOutput::from(f)).collect::<Vec<UserOutput>>())?)
            
    }

    pub async fn create_aluno(&self, mut user: Box<UserInput>) -> Result<Option<UserOutput>, AppError> {
        let mut user = CreateUserValidation::validate(&mut (user))?;
        user.password = bcrypt::hash(user.password)?;
        user.user_type = "aluno".to_string();
        Ok(self.repository.create(user).await
            .map(|opt_user| opt_user.map(|user| UserOutput::from(user)))?)
    }

    pub async fn update_aluno(
        &self,
        mut user: Box<OptionUserSchema>,
        id: &ObjectId,
    ) -> Result<Option<UserOutput>, AppError> {
        UpdateUserValidation::validate(&mut (user))?;
        Ok(self.repository.update_one(user, id).await.map(|op|op.map(|item|UserOutput::from(item)))?)
    }

    pub async fn delete_aluno(&self, id: &ObjectId) -> Result<bool, AppError> {
        Ok(self
            .repository
            .delete_one(id)
            .await?)
    }
}
