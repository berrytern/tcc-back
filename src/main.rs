mod application;
mod di;
mod controllers;
mod port;
mod routes;
mod infrastructure;
mod utils;
mod errors;
use actix_web::{App, HttpServer, web::{Data,get,post,patch,delete}};
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;
use crate::di::d_injection::build;
use routes::auth::login;
use routes::aluno::{get_aluno,create_aluno,update_aluno,delete_aluno, get_all_aluno};
use routes::gestor::{get_gestor,create_gestor,update_gestor,delete_gestor, get_all_gestor};
use routes::professor::{get_professor,create_professor,update_professor,delete_professor, get_all_professor};
use routes::solicitacao::{get_one_solicitacao,create_solicitacao,update_solicitacao,delete_solicitacao, get_all_solicitacao};
use routes::turma::{get_one_turma,create_turma,update_turma,delete_turma, get_all_turma};
use utils::settings::load_env;
use utoipa::{OpenApi};

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


#[derive(OpenApi)]
#[openapi(paths(routes::auth::login))]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = load_env();
    let app = build(&env).await;


    HttpServer::new(move || {
        println!("running");
        let mut app =App::new()
            .app_data(Data::new(app.clone()));
        let (app, api) = app
            .into_utoipa_app()
            .service(login)
            .service(get_all_aluno)
            .service(create_aluno)
            .service(get_aluno)
            .service(update_aluno)
            .service(delete_aluno)
            .service(get_all_gestor)
            .service(create_gestor)
            .service(get_gestor)
            .service(update_gestor)
            .service(delete_gestor)
            .service(get_all_professor)
            .service(create_professor)
            .service(get_professor)
            .service(update_professor)
            .service(delete_professor)
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
            .split_for_parts();
        app.service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", api),
            )
        
    })
    .bind(("0.0.0.0", env.port))?
    .workers(env.workers.into()).run()
    .await
}