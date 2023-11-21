use crate::application::validation::{
    create_user_validation::CreateUserValidation, update_user_validation::UpdateUserValidation,
};
use crate::{
    errors::AppError,
    infrastructure::{
        database::schemas::user_schema::{OptionUser, User},
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

    pub async fn get_one(&self, user: &mut OptionUser) -> Result<Option<User>, AppError> {
        Ok(self.repository.get_one(user).await?)
    }
    pub async fn get_all_aluno(
        &self,
        user: &mut OptionUser,
        options: QueryOptions,
    ) -> Result<Vec<User>, AppError> {
        Ok(self
            .repository
            .get_all(user, options)
            .await?)
    }

    pub async fn create_aluno(&self, mut user: Box<User>) -> Result<Option<Box<User>>, AppError> {
        CreateUserValidation::validate(&mut (*user))?;
        user.password = bcrypt::hash(user.password)?;
        user.user_type = "aluno".to_string();
        Ok(self.repository.create(user).await?)
    }

    pub async fn update_aluno(
        &self,
        mut user: Box<OptionUser>,
        id: &ObjectId,
    ) -> Result<Option<User>, AppError> {
        UpdateUserValidation::validate(&mut (*user))?;
        Ok(self.repository.update_one(user, id).await?)
    }

    pub async fn delete_aluno(&self, id: &ObjectId) -> Result<bool, AppError> {
        Ok(self
            .repository
            .delete_one(id)
            .await?)
    }
}
