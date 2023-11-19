use mongodb::{
    bson::{doc, extjson::de::Error as BsonError, oid::ObjectId, to_bson, Document},
    results::{UpdateResult,DeleteResult},
    options::FindOptions,
    Client, options::ClientOptions,
};
use mongodb::error::Error as MongoDbError;
use futures::stream::TryStreamExt;


use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub async fn get_connection(uri: &str) -> Result<Client, mongodb::error::Error> {
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.app_name = Some("My App".to_string());
    return Client::with_options(client_options);
}
#[derive(Clone)]
pub struct Model<T>{
    collection: mongodb::Collection<T>,
}

impl<T> Model<T>  {
    pub async fn new(db: mongodb::Database, collection_name: &str) ->  Self {
        Model::<T>{
            collection: db.collection::<T>(collection_name)
        }
    }

    pub async fn find_one(&self, filter: Document) -> Result<Option<T>, BsonError>
    where
    T: DeserializeOwned + Unpin + Send + Sync,
    {
        let result= self.collection.find_one(
            filter, None
        ).await.ok().expect("Error on creating operation");
        Ok(result)
    }

    pub async fn find(&self, filter: Document) -> Result<Vec<T>, BsonError>
    where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize + std::fmt::Debug,
    {
        let mut result: Vec<T> = Vec::new();
        let options = FindOptions::builder().limit(10).build();
        let mut cursor= self.collection.find(
            filter, options
        ).await.ok().expect("Error on find operation");
        
        while let Ok(Some(item)) = cursor.try_next().await {
            result.push(item)
        }
        Ok(result)
    }

    pub async fn create(&self, data: &T) -> Result<Option<ObjectId>, MongoDbError> 
    where
    T: Serialize,
    {
        self.collection.insert_one(
            data, None
        ).await
            .map(|op| Some(op.inserted_id.as_object_id().unwrap()))
    }

    pub async fn delete_one(&self, filter: Document) -> Result<bool, MongoDbError>
    where
    T: Serialize,
    {
        self.collection.delete_one(filter, None).await
            .map(|result| result.deleted_count != 0)
    }

}
impl<T> Model<T>{
    pub async fn update_one<G>(&self, data: G, id: ObjectId) -> Result<UpdateResult, BsonError> 
    where
    G: Serialize + std::convert::From<G>,
    {
        let d = to_bson::<G>(&data).ok().expect("Error on bson conversion");
        let result= self.collection.update_one(
            doc!{ "_id": id }, doc!{"$set": d}, None
        ).await.ok().expect("Error on creating operation");
        Ok(result)
    }
}