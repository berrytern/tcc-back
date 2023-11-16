use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId, to_bson, to_document, Document},
    results::{InsertOneResult,UpdateResult,DeleteResult},
    options::FindOptions,
    Client, options::ClientOptions,
    Collection,
};

use futures::stream::TryStreamExt;


use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub async fn get_connection(uri: &str) -> Result<Client, mongodb::error::Error> {
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.app_name = Some("My App".to_string());
    return Client::with_options(client_options);
}
pub struct Model<T>{
    collection: mongodb::Collection<T>,
}

impl<T> Model<T>  {
    pub async fn new (db: mongodb::Database, collection_name: &str) ->  Self {
        Model::<T>{
            collection: db.collection::<T>(collection_name)
        }
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<T>, Error>
    where
    T: DeserializeOwned + Unpin + Send + Sync,
    {
        let result= self.collection.find_one(
            doc!{"_id": id}, None
        ).await.ok().expect("Error on creating operation");
        Ok(result)
    }

    pub async fn find(&self, filter: Document) -> Result<Vec<T>, Error>
    where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize,
    {
        let mut result: Vec<T> = Vec::new();
        let options = FindOptions::builder().limit(10).build();
        let mut cursor= self.collection.find(
            filter, options
        ).await.ok().expect("Error on creating operation");
        while let Some(book) = cursor.try_next().await.ok().expect("Iteration Error") {
            result.push(book)
        }
        Ok(result)
    }

    pub async fn create(&self, data: T) -> Result<InsertOneResult, Error> 
    where
    T: Serialize,
    {
        let result= self.collection.insert_one(
            data, None
        ).await.ok().expect("Error on creating operation");
        Ok(result)
    }

    pub async fn update_one(&self, data: T, id: ObjectId) -> Result<UpdateResult, Error> 
    where
    T: Serialize + std::convert::From<T>,
    {
        let d = to_bson::<T>(&data).ok().expect("Error on bson conversion");
        let result= self.collection.update_one(
            doc!{ "_id": id }, doc!{"$set": d}, None
        ).await.ok().expect("Error on creating operation");
        Ok(result)
    }

    pub async fn delete_one(&self, id: ObjectId) -> Result<DeleteResult, Error> 
    where
    T: Serialize,
    {
        let result= self.collection.delete_one(
            doc!{ "_id": id }, None
        ).await.ok().expect("Error on creating operation");
        Ok(result)
    }

}
