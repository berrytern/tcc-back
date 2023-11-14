mod di;
mod controllers;
mod port;
use std::env;
use dotenv::dotenv;
use actix_web::{App, HttpResponse, HttpServer, Responder};
use controllers::gestor_controller::GestorController;
use crate::di::d_injection::build;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = build().await;
    dotenv().ok();
    let workers = match env::var("WORKERS"){
        Ok(var) => var,
        Err(_error) => panic!("Environment variable 'WORKERS' not setted")
    };
    HttpServer::new(|| {
        println!("running");
        
        App::new()
            .route("/hey", || app.controllers.gestor.get_one())
    })
    .bind(("0.0.0.0", 8080))?
    .workers(workers.to_string().parse::<usize>().unwrap()).run()
    .await
}