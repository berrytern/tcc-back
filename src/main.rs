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
use routes::auth::login;
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
            .route("/v1/auth", post().to(login))
            .route("/v1/alunos", get().to(get_all_aluno))
            .route("/v1/alunos", post().to(create_aluno))
            .route("/v1/alunos/{id}", get().to(get_aluno))
            .route("/v1/alunos/{id}", patch().to(update_aluno))
            .route("/v1/alunos/{id}", delete().to(delete_aluno))
            .route("/v1/gestores", get().to(get_all_gestor))
            .route("/v1/gestores", post().to(create_gestor))
            .route("/v1/gestores/{id}", get().to(get_gestor))
            .route("/v1/gestores/{id}", patch().to(update_gestor))
            .route("/v1/gestores/{id}", delete().to(delete_gestor))
            .route("/v1/professores", get().to(get_all_professor))
            .route("/v1/professores", post().to(create_professor))
            .route("/v1/professores/{id}", get().to(get_professor))
            .route("/v1/professores/{id}", patch().to(update_professor))
            .route("/v1/professores/{id}", delete().to(delete_professor))
            .route("/v1/turma", get().to(get_all_turma))
            .route("/v1/turma", post().to(create_turma))
            .route("/v1/turma/one", get().to(get_one_turma))
            .route("/v1/turma/{aluno_id}/{professor_id}", patch().to(update_turma))
            .route("/v1/turma/{aluno_id}/{professor_id}", delete().to(delete_turma))
            .route("/v1/solicitacoes", get().to(get_all_solicitacao))
            .route("/v1/solicitacoes", post().to(create_solicitacao))
            .route("/v1/solicitacoes/one", get().to(get_one_solicitacao))
            .route("/v1/solicitacoes/{aluno_id}/{professor_id}", patch().to(update_solicitacao))
            .route("/v1/solicitacoes/{aluno_id}/{professor_id}", delete().to(delete_solicitacao))
    })
    .bind(("0.0.0.0", 8080))?
    .workers(env.workers.into()).run()
    .await
}