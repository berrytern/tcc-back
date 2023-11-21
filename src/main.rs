mod application;
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
use routes::solicitacao::{get_one_solicitacao,create_solicitacao,update_solicitacao,delete_solicitacao, get_all_solicitacao};
use routes::turma::{get_one_turma,create_turma,update_turma,delete_turma, get_all_turma};
use utils::settings::load_env;

/*async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}*/

/*#[tokio::main]
async fn main() {
    let client = get_connection("mongodb://admin:admin@localhost:27017/").await.expect("Cannot connect to MongoDb");
    let db = client.database("teste");
    let userModel = Model::<User>::new(db, "users").await;
    
    let result = userModel.find(doc!{
        "name": "jose".to_string()}.into()).await.expect("err");
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
            .route("/alunos", get().to(get_all_aluno))
            .route("/alunos", post().to(create_aluno))
            .route("/alunos/{id}", get().to(get_aluno))
            .route("/alunos/{id}", patch().to(update_aluno))
            .route("/alunos/{id}", delete().to(delete_aluno))
            .route("/gestores", get().to(get_all_gestor))
            .route("/gestores", post().to(create_gestor))
            .route("/gestores/{id}", get().to(get_gestor))
            .route("/gestores/{id}", patch().to(update_gestor))
            .route("/gestores/{id}", delete().to(delete_gestor))
            .route("/professores", get().to(get_all_professor))
            .route("/professores", post().to(create_professor))
            .route("/professores/{id}", get().to(get_professor))
            .route("/professores/{id}", patch().to(update_professor))
            .route("/professores/{id}", delete().to(delete_professor))
            .route("/turma", get().to(get_all_turma))
            .route("/turma", post().to(create_turma))
            .route("/turma/one", get().to(get_one_turma))
            .route("/turma/{aluno_id}/{professor_id}", patch().to(update_turma))
            .route("/turma/{aluno_id}/{professor_id}", delete().to(delete_turma))
            .route("/solicitacoes", get().to(get_all_solicitacao))
            .route("/solicitacoes", post().to(create_solicitacao))
            .route("/solicitacoes/one", get().to(get_one_solicitacao))
            .route("/solicitacoes/{aluno_id}/{professor_id}", patch().to(update_solicitacao))
            .route("/solicitacoes/{aluno_id}/{professor_id}", delete().to(delete_solicitacao))
    })
    .bind(("0.0.0.0", 8080))?
    .workers(env.workers.into()).run()
    .await
}