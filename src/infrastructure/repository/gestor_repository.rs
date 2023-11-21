use mongodb::bson::oid::ObjectId;
use mongodb::{IndexModel,
    bson::{DateTime,to_document, doc, extjson::de::Error as BsonError},
    options::IndexOptions};
use mongodb::error::Error as MongoDbError;
use crate::infrastructure::database::{schemas::user_schema::{User,OptionUser},connection::Model};
use crate::port::query_filter::QueryOptions;

#[derive(Clone)]
pub struct GestorRepository{
    model: Box<Model<User>>,
}
impl GestorRepository {
    pub async fn new(model: Box<Model<User>>)-> Self{
        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder().keys(doc!{"email":1}).options(options).build();
        let _ = model.create_index(index, None).await;
        GestorRepository {
            model
        }
    }
    pub async fn get_one(&self, user: &OptionUser) -> Result<Option<User>,BsonError> {
        let filter = to_document(user).expect("error converting to document");
        self.model.find_one(filter).await
    }
    pub async fn get_all(&self, mut user: OptionUser, options: QueryOptions) -> Result<Vec<User>,BsonError> {
        user.user_type = Some("gestor".to_string());
        let filter = to_document(&user).expect("error converting to document");
        self.model.find(filter, options).await
    }
    pub async fn create<'a>(&self, mut user: Box<User>) ->  Result<Option<Box<User>>,MongoDbError> {
        user.id = None;
        user.matricula = None;
        user.user_type = "gestor".to_string();
        user.created_at = Some(DateTime::now());
        user.updated_at = Some(DateTime::now());
        self.model.create(&user).await.map(|op| {
            user.id = op;
            Some(user)
        })
    }
    pub async fn update_one<'a>(&self, mut user: Box<OptionUser>, id: &ObjectId) ->  Result<Option<User>,BsonError> {
        user.id = None;
        user.user_type = None;
        user.matricula = None;
        user.created_at = None;
        user.updated_at = Some(DateTime::now());
        let filter = doc!{"_id":id};
        match self.model.update_one(user, filter).await {
            Ok(up) => {
                if up.matched_count != 0 {
                    self.model.find_one(doc!{"_id": id}).await
                } else{
                    Ok(None)
                }
            },
            Err(err) => Err(err),
        }
    }
    pub async fn delete_one<'a>(&self, id: &ObjectId) -> Result<bool,MongoDbError> {
        let filter = doc!{"_id": id, "user_type": "gestor"};
        self.model.delete_one(filter).await
    }
}