use crate::application::models::aluno::{AlunoUpdateModel, CreateAlunoModel};
use crate::application::models::user::UserOutput;
use crate::infrastructure::database::schemas::user_schema::UserSchema;
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
        Ok(self.repository.get_one(user).await.map(|op| op.map(UserOutput::from))?)
    }
    pub async fn get_all_aluno(
        &self,
        user: &mut OptionUserSchema,
        options: QueryOptions,
    ) -> Result<Vec<UserOutput>, AppError> {
        Ok(self
            .repository
            .get_all(user, options).await.map( |item| item.into_iter().map(UserOutput::from).collect::<Vec<UserOutput>>())?)
            
    }

    pub async fn create_aluno(&self, user: CreateAlunoModel) -> Result<Option<UserOutput>, AppError> {
        let mut user: UserSchema = user.into();
        user.password = bcrypt::hash(user.password)?;
        Ok(self.repository.create(user).await
            .map(|opt_user| opt_user.map(UserOutput::from))?)
    }

    pub async fn update_aluno(
        &self,
        user: Box<AlunoUpdateModel>,
        id: &ObjectId,
    ) -> Result<Option<UserOutput>, AppError> {
        let user: OptionUserSchema = (*user).into();
        Ok(self.repository.update_one(&user, id).await.map(|op|op.map(UserOutput::from))?)
    }

    pub async fn delete_aluno(&self, id: &ObjectId) -> Result<bool, AppError> {
        Ok(self
            .repository
            .delete_one(id)
            .await?)
    }
}
