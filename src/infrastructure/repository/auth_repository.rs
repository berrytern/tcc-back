use mongodb::{IndexModel,
    bson::{to_document, doc, extjson::de::Error as BsonError},
    options::{IndexOptions, UpdateOptions}};
use mongodb::error::Error as MongoDbError;
use crate::infrastructure::database::{schemas::auth_schema::{Auth,OptionAuth},connection::Model};
use crate::port::query_filter::QueryOptions;

#[derive(Clone)]
pub struct AuthRepository{
    model: Box<Model<Auth>>,
}
impl AuthRepository {
    pub async fn new(model: Box<Model<Auth>>)-> Self{
        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder().keys(doc!{"email":1}).options(options).build();
        let _ = model.create_index(index, None).await;
        AuthRepository {
            model
        }
    }
    pub async fn get_one(&self, auth: &OptionAuth) -> Result<Option<Auth>,BsonError> {
        let filter = to_document(auth).expect("error converting to document");
        self.model.find_one(filter).await
    }
    pub async fn get_all(&self, auth: OptionAuth, options: QueryOptions) -> Result<Vec<Auth>,BsonError> {
        let filter = to_document(&auth).expect("error converting to document");
        self.model.find(filter, options).await
    }
    pub async fn create<'a>(&self, mut auth: Box<Auth>) ->  Result<Option<Box<Auth>>,MongoDbError> {
        self.model.create(&auth).await.map(|op| {
            auth.id = op;
            Some(auth)
        })
    }
    pub async fn update_one<'a>(&self, auth: Box<OptionAuth>, email: &String) ->  Result<Option<Auth>,BsonError> {
        let filter = doc!{"email":email};
        let options = Some(UpdateOptions::builder().upsert(true).build());
        match self.model.update_one(auth, filter, options).await {
            Ok(up) => {
                if up.matched_count != 0 {
                    self.model.find_one(doc!{"email": email}).await
                } else{
                    Ok(None)
                }
            },
            Err(err) => Err(err),
        }
    }
    pub async fn delete_one<'a>(&self, email: &String) -> Result<bool,MongoDbError> {
        let filter = doc!{"email": email};
        self.model.delete_one(filter).await
    }
}