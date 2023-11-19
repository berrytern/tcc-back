mod di;
mod controllers;
mod port;
mod routes;
mod infrastructure;
mod utils;
mod errors;
use actix_web::{App, HttpServer, web::{Data,get,post,patch,delete}};
use crate::di::d_injection::build;
use routes::aluno::{get_aluno,create_aluno,update_aluno,delete_aluno, get_all_aluno};
use routes::gestor::{get_gestor,create_gestor,update_gestor,delete_gestor, get_all_gestor};
use routes::professor::{get_professor,create_professor,update_professor,delete_professor, get_all_professor};
use utils::settings::load_env;

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
    let env = load_env();
    let app = build(&env).await;

    HttpServer::new(move || {
        println!("running");
        App::new()
            .app_data(Data::new(app.clone()))
            .route("/aluno", get().to(get_all_aluno))
            .route("/aluno", post().to(create_aluno))
            .route("/aluno/{id}", get().to(get_aluno))
            .route("/aluno/{id}", patch().to(update_aluno))
            .route("/aluno/{id}", delete().to(delete_aluno))
            .route("/gestor", get().to(get_all_gestor))
            .route("/gestor", post().to(create_gestor))
            .route("/gestor/{id}", get().to(get_gestor))
            .route("/gestor/{id}", patch().to(update_gestor))
            .route("/gestor/{id}", delete().to(delete_gestor))
            .route("/professor", get().to(get_all_professor))
            .route("/professor", post().to(create_professor))
            .route("/professor/{id}", get().to(get_professor))
            .route("/professor/{id}", patch().to(update_professor))
            .route("/professor/{id}", delete().to(delete_professor))
    })
    .bind(("0.0.0.0", 8080))?
    .workers(env.workers.into()).run()
    .await
}