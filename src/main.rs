mod di;
mod controllers;
mod port;
mod routes;
mod infrastructure;
use std::env;
use dotenv::dotenv;
use actix_web::{App, HttpResponse, HttpServer, Responder};
use controllers::gestor_controller::GestorController;
use crate::di::d_injection::build;
use routes::gestor::{get_gestor,create_gestor,update_gestor,delete_gestor};
use infrastructure::database::{schemas::user_schema::User,connection::{Model, get_connection}};
use mongodb::bson::doc;

/*async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}*/

/*#[tokio::main]
async fn main() {
    let client = get_connection("mongodb://admin:admin@localhost:27017/").await.ok().expect("Cannot connect to MongoDb");
    let db = client.database("teste");
    let userModel = Model::<User>::new(db, "users").await;
    
    let result = userModel.find(doc!{
        "name": "jose".to_string()}.into()).await.ok().expect("err");
    println!("result: {:?}", result);
}*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = build().await;
    dotenv().ok();
    let workers = match env::var("WORKERS"){
        Ok(var) => var,
        Err(_error) => panic!("Environment variable 'WORKERS' not setted")
    };
    let mongodbURI = match env::var("MONGODB_URI"){
        Ok(var) => var,
        Err(_error) => panic!("Environment variable 'WORKERS' not setted")
    };

    let client = get_connection(&mongodbURI).await.ok().expect("Cannot connect to MongoDb");
    let db = client.database("teste");
    let userModel = Model::<User>::new(db, "users").await;
    HttpServer::new(|| {
        println!("running");

        App::new()
            .route("/hey", app.controllers.gestor.get_one())
    })
    .bind(("0.0.0.0", 8080))?
    .workers(workers.to_string().parse::<usize>().unwrap()).run()
    .await
}