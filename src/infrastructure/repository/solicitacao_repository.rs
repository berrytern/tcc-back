use mongodb::bson::oid::ObjectId;
use mongodb::{IndexModel,
    bson::{to_document, doc, extjson::de::Error as BsonError},
    options::IndexOptions};
use mongodb::error::Error as MongoDbError;
use crate::errors::AppError;
use crate::infrastructure::database::{schemas::solicitacao_schema::{SolicitacaoSchema,OptionSolicitacaoSchema},connection::RepoModel};
use crate::port::query_filter::QueryOptions;

#[derive(Clone)]
pub struct SolicitacaoRepository{
    model: RepoModel<SolicitacaoSchema>,
}
impl SolicitacaoRepository {
    pub async fn new(model: RepoModel<SolicitacaoSchema>)-> Self{
        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder().keys(doc!{"id_aluno":1,"id_professor":1}).options(options).build();
        let _ = model.create_index(index, None).await;
        SolicitacaoRepository {
            model
        }
    }
    pub async fn get_one(&self, solicitacao: &OptionSolicitacaoSchema) -> Result<Option<SolicitacaoSchema>,BsonError> {
        let filter = to_document(solicitacao).expect("error converting to document");
        self.model.find_one(filter).await
    }
    pub async fn get_all(&self, solicitacao: &OptionSolicitacaoSchema, options: QueryOptions) -> Result<Vec<SolicitacaoSchema>,BsonError> {
        let filter = to_document(solicitacao).expect("error converting to document");
        self.model.find(filter, options).await
    }
    pub async fn create(&self, solicitacao: &mut SolicitacaoSchema) ->  Result<(), AppError> {
        Ok(self.model.create(&solicitacao).await.map(|op_id| {
            solicitacao.id = op_id;
        })?)
    }
    pub async fn update_one(&self, solicitacao: OptionSolicitacaoSchema, aluno_id: &ObjectId, prof_id: &ObjectId) ->  Result<Option<SolicitacaoSchema>,AppError> {
        let filter = doc!{"aluno_id":aluno_id,"professor_id":prof_id};
        match self.model.update_one(solicitacao, filter, None).await {
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
    pub async fn delete_one(&self, aluno_id: &ObjectId, prof_id: &ObjectId) -> Result<bool,MongoDbError> {
        let filter = doc!{"id_aluno": aluno_id, "id_professor": prof_id};
        self.model.delete_one(filter).await
    }
}