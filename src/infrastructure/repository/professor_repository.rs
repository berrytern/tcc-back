use mongodb::bson::oid::ObjectId;
use mongodb::bson::{to_document, doc, extjson::de::Error as BsonError};
use mongodb::error::Error as MongoDbError;
use crate::infrastructure::database::{schemas::user_schema::{User,OptionUser},connection::Model};


#[derive(Clone)]
pub struct ProfessorRepository{
    model: Box<Model<User>>,
}
impl ProfessorRepository {
    pub fn new(model: Box<Model<User>>)-> Self{
        ProfessorRepository {
            model
        }
    }
    pub async fn get_one(&self, user: OptionUser) -> Result<Option<User>,BsonError> {
        let filter = to_document(&user).ok().expect("error converting to document");
        self.model.find_one(filter).await
    }
    pub async fn get_all(&self, mut user: OptionUser, ) -> Result<Vec<User>,BsonError> {
        user.user_type = Some("professor".to_string());
        let filter = to_document(&user).ok().expect("error converting to document");
        self.model.find(filter).await
    }
    pub async fn create<'a>(&self, mut user: Box<User>) ->  Result<Option<Box<User>>,MongoDbError> {
        user.id = None;
        user.user_type = "professor".to_string();
        self.model.create(&user).await.map(|op| {
            user.id = op;
            return Some(user);
        })
    }
    pub async fn update_one<'a>(&self, mut user: Box<OptionUser>, id: ObjectId) ->  Result<Option<User>,BsonError> {
        user.id = None;
        user.user_type = None;
        match self.model.update_one(user, id).await {
            Ok(up) => {
                if up.upserted_id.is_some() {
                    return self.model.find_one(doc!{"_id": Some(up.upserted_id)}).await
                        .map(|us| us);
                } else{
                    return Ok(None);
                }
            },
            Err(err) => Err(err),
        }
    }
    pub async fn delete_one<'a>(&self, id: ObjectId) -> Result<bool,MongoDbError> {
        let filter = doc!{"_id": id, "user_type": "professor"};
        self.model.delete_one(filter).await
    }
}