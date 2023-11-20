use mongodb::bson::oid::ObjectId;
use mongodb::{IndexModel,
    bson::{DateTime,to_document, doc, extjson::de::Error as BsonError},
    options::IndexOptions};
use mongodb::error::Error as MongoDbError;
use crate::infrastructure::database::{schemas::turma_schema::{Turma,OptionTurma},connection::Model};
use crate::port::query_filter::QueryOptions;

#[derive(Clone)]
pub struct TurmaRepository{
    model: Box<Model<Turma>>,
}
impl TurmaRepository {
    pub async fn new(model: Box<Model<Turma>>)-> Self{
        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder().keys(doc!{"id_aluno":1,"id_professor":1}).options(options).build();
        let _ = model.create_index(index, None).await;
        TurmaRepository {
            model
        }
    }
    pub async fn get_one(&self, turma: OptionTurma) -> Result<Option<Turma>,BsonError> {
        let filter = to_document(&turma).expect("error converting to document");
        self.model.find_one(filter).await
    }
    pub async fn get_all(&self, turma: OptionTurma, options: QueryOptions) -> Result<Vec<Turma>,BsonError> {
        let filter = to_document(&turma).expect("error converting to document");
        self.model.find(filter, options).await
    }
    pub async fn create<'a>(&self, mut turma: Box<Turma>) ->  Result<Option<Box<Turma>>,MongoDbError> {
        turma.created_at = Some(DateTime::now());
        turma.updated_at = Some(DateTime::now());
        self.model.create(&turma).await.map(|_| {
            Some(turma)
        })
    }
    pub async fn update_one<'a>(&self, mut turma: Box<OptionTurma>, aluno_id: ObjectId, prof_id: ObjectId) ->  Result<Option<Turma>,BsonError> {
        turma.id_aluno = None;
        turma.id_professor = None;
        turma.created_at = None;
        turma.updated_at = Some(DateTime::now());
        let filter = doc!{"aluno_id":aluno_id,"professor_id":prof_id};
        match self.model.update_one(turma, filter).await {
            Ok(up) => {
                if up.matched_count != 0 {
                    self.model.find_one(doc!{"aluno_id": aluno_id, "professor_id":prof_id}).await
                } else{
                    Ok(None)
                }
            },
            Err(err) => Err(err),
        }
    }
    pub async fn delete_one<'a>(&self, aluno_id: ObjectId, prof_id: ObjectId) -> Result<bool,MongoDbError> {
        let filter = doc!{"id_aluno": aluno_id, "id_professor": prof_id};
        self.model.delete_one(filter).await
    }
}