use mongodb::{IndexModel,
    bson::{to_document, doc, extjson::de::Error as BsonError, oid::ObjectId},
    options::{IndexOptions, UpdateOptions}};
use mongodb::error::Error as MongoDbError;
use crate::infrastructure::database::{schemas::auth_schema::Auth,connection::RepoModel};
use crate::port::query_filter::QueryOptions;

#[derive(Clone)]
pub struct AuthRepository{
    model: Box<RepoModel<Auth>>,
}
impl AuthRepository {
    pub async fn new(model: Box<RepoModel<Auth>>)-> Self{
        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder().keys(doc!{"email":1}).options(options).build();
        let _ = model.create_index(index, None).await;
        AuthRepository {
            model
        }
    }
    pub async fn get_one(&self, auth: &Auth) -> Result<Option<Auth>,BsonError> {
        let filter = to_document(auth).expect("error converting to document");
        self.model.find_one(filter).await
    }
    pub async fn get_all(&self, auth: Auth, options: QueryOptions) -> Result<Vec<Auth>,BsonError> {
        let filter = to_document(&auth).expect("error converting to document");
        self.model.find(filter, options).await
    }
    pub async fn create<'a>(&self, mut auth: Box<Auth>) ->  Result<Option<Box<Auth>>,MongoDbError> {
        self.model.create(&auth).await.map(|op| {
            if let Some(id) = op {
                auth.id = id;
            }
            Some(auth)
        })
    }
    pub async fn update_one<'a>(&self, auth: Box<Auth>, id: &ObjectId) ->  Result<Option<Auth>,BsonError> {
        let filter = doc!{"_id":id};
        let options = Some(UpdateOptions::builder().upsert(true).build());
        match self.model.update_one(auth, filter, options).await {
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
    pub async fn delete_one<'a>(&self, email: &String) -> Result<bool,MongoDbError> {
        let filter = doc!{"email": email};
        self.model.delete_one(filter).await
    }
}