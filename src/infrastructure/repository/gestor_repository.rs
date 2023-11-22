use mongodb::bson::oid::ObjectId;
use mongodb::{IndexModel,
    bson::{to_document, doc, extjson::de::Error as BsonError},
    options::IndexOptions};
use mongodb::error::Error as MongoDbError;
use crate::infrastructure::database::{schemas::user_schema::{UserSchema,OptionUserSchema},connection::RepoModel};
use crate::port::query_filter::QueryOptions;

#[derive(Clone)]
pub struct GestorRepository{
    model: Box<RepoModel<UserSchema>>,
}
impl GestorRepository {
    pub async fn new(model: Box<RepoModel<UserSchema>>)-> Self{
        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder().keys(doc!{"email":1}).options(options).build();
        let _ = model.create_index(index, None).await;
        GestorRepository {
            model
        }
    }
    pub async fn get_one(&self, user: &mut OptionUserSchema) -> Result<Option<UserSchema>,BsonError> {
        user.user_type = Some("gestor".to_string());
        let filter = to_document(user).expect("error converting to document");
        self.model.find_one(filter).await
    }
    pub async fn get_all(&self, user: &mut OptionUserSchema, options: QueryOptions) -> Result<Vec<UserSchema>,BsonError> {
        user.user_type = Some("gestor".to_string());
        let filter = to_document(&user).expect("error converting to document");
        self.model.find(filter, options).await
    }
    pub async fn create<'a>(&self, mut user: Box<UserSchema>) ->  Result<Option<Box<UserSchema>>,MongoDbError> {
        self.model.create(&user).await.map(|op| {
            user.id = op;
            Some(user)
        })
    }
    pub async fn update_one<'a>(&self, user: Box<OptionUserSchema>, id: &ObjectId) ->  Result<Option<UserSchema>,BsonError> {

        let filter = doc!{"_id":id};
        match self.model.update_one(user, filter, None).await {
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