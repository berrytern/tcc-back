use std::convert::Infallible;

use actix_web::{Responder, HttpResponse};
use mongodb::bson::oid::ObjectId;
use crate::infrastructure::database::{schemas::user_schema::User,connection::Model};


pub async fn get_gestor(user_model: Model<User>, id: ObjectId) -> HttpResponse {
    let result = user_model.get_by_id(id).await;
    if let Err(err) = result {
        HttpResponse::InternalServerError().body(err.to_string())
    } else{
        let body = result.unwrap();
        HttpResponse::Ok().json(if body.is_some() { Some(body) } else { None })
    }
}

pub async fn get_all_gestor(user_model: Model<User>, id: ObjectId) -> HttpResponse {
    let result = user_model.find(id).await;
    if let Err(err) = result {
        HttpResponse::InternalServerError().body(err.to_string())
    } else{
        let body = result.unwrap();
        HttpResponse::Ok().json(body)
    }
}

pub async fn create_gestor(user_model: Model<User>, user: User) -> HttpResponse {
    let result = user_model.create(user).await;
    if let Err(err) = result {
        HttpResponse::InternalServerError().body(err.to_string())
    } else{
        let body: User= result.unwrap().into();
        return HttpResponse::Created().json(body);
    }
}

pub async fn update_gestor() -> HttpResponse {
    HttpResponse::Ok().body("body")

}

pub async fn delete_gestor(user_model: Model<User>, id: ObjectId) -> HttpResponse {
    let result = user_model.delete_one(id).await;
    if let Err(err) = result {
        HttpResponse::InternalServerError().body(err.to_string())
    } else{
        HttpResponse::Ok().body("body")
    }
}