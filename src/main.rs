mod application;
mod di;
mod controllers;
mod port;
mod routes;
mod infrastructure;
mod utils;
mod errors;
use actix_web::{App, HttpServer, web::Data};
use once_cell::sync::Lazy;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;
use crate::di::d_injection::build;
use crate::utils::settings::Env;
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
pub static ENV: Lazy<Env> = Lazy::new(|| load_env());

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = load_env();
    let app = build(&ENV).await;


    HttpServer::new(move || {
        let app =App::new()
            .app_data(Data::new(app.clone()));
        let (app, mut api) = app
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
            .service(get_all_turma)
            .service(create_turma)
            .service(get_one_turma)
            .service(update_turma)
            .service(delete_turma)
            .service(get_all_solicitacao)
            .service(create_solicitacao)
            .service(get_one_solicitacao)
            .service(update_solicitacao)
            .service(delete_solicitacao)
            .split_for_parts();
        api.info.title = "TCC API".to_string();
        api.info.contact = Some(utoipa::openapi::ContactBuilder::new()
            .name(Some("João M. C. Hluchan"))
            .email(Some("berrytern@gmail.com"))
            .build());
        app.service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", api),
            )
        
    })
    .bind(("0.0.0.0", env.port))?
    .workers(env.workers.into()).run()
    .await
}