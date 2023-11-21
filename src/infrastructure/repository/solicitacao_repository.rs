use mongodb::bson::oid::ObjectId;
use mongodb::{IndexModel,
    bson::{DateTime,to_document, doc, extjson::de::Error as BsonError},
    options::IndexOptions};
use mongodb::error::Error as MongoDbError;
use crate::errors::AppError;
use crate::infrastructure::database::schemas::solicitacao_schema::StatusType;
use crate::infrastructure::database::{schemas::solicitacao_schema::{Solicitacao,OptionSolicitacao},connection::Model};
use crate::port::query_filter::QueryOptions;

#[derive(Clone)]
pub struct SolicitacaoRepository{
    model: Box<Model<Solicitacao>>,
}
impl SolicitacaoRepository {
    pub async fn new(model: Box<Model<Solicitacao>>)-> Self{
        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder().keys(doc!{"id_aluno":1,"id_professor":1}).options(options).build();
        let _ = model.create_index(index, None).await;
        SolicitacaoRepository {
            model
        }
    }
    pub async fn get_one(&self, solicitacao: &OptionSolicitacao) -> Result<Option<Solicitacao>,BsonError> {
        let filter = to_document(solicitacao).expect("error converting to document");
        self.model.find_one(filter).await
    }
    pub async fn get_all(&self, solicitacao: &OptionSolicitacao, options: QueryOptions) -> Result<Vec<Solicitacao>,BsonError> {
        let filter = to_document(solicitacao).expect("error converting to document");
        self.model.find(filter, options).await
    }
    pub async fn create<'a>(&self, mut solicitacao: Box<Solicitacao>) ->  Result<Option<Box<Solicitacao>>,AppError> {
        StatusType::validate(&solicitacao.status)?;
        solicitacao.status = "pending".to_string();
        solicitacao.created_at = Some(DateTime::now());
        solicitacao.updated_at = Some(DateTime::now());
        Ok(self.model.create(&solicitacao).await.map(|_| {
            Some(solicitacao)
        })?)
    }
    pub async fn update_one<'a>(&self, mut solicitacao: Box<OptionSolicitacao>, aluno_id: &ObjectId, prof_id: &ObjectId) ->  Result<Option<Solicitacao>,AppError> {
        solicitacao.id_aluno = None;
        solicitacao.id_professor = None;
        solicitacao.created_at = None;
        solicitacao.updated_at = Some(DateTime::now());
        if let Some(status) = &solicitacao.status{
            StatusType::validate(&status)?;
        }
        let filter = doc!{"aluno_id":aluno_id,"professor_id":prof_id};
        match self.model.update_one(solicitacao, filter).await {
            Ok(up) => {
                if up.matched_count != 0 {
                    Ok(self.model.find_one(doc!{"aluno_id": aluno_id, "professor_id":prof_id}).await?)
                } else{
                    Ok(None)
                }
            },
            Err(err) => Err(AppError::from(err)),
        }
    }
    pub async fn delete_one<'a>(&self, aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<bool,MongoDbError> {
        let filter = doc!{"id_aluno": aluno_id, "id_professor": prof_id};
        self.model.delete_one(filter).await
    }
}