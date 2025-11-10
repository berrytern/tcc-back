use std::borrow::BorrowMut;
use std::sync::Arc;

use amqp_client_rust::api::eventbus::AsyncEventbusRabbitMQ;
use amqp_client_rust::domain::config::{Config, ConfigOptions};
use redis::aio::MultiplexedConnection;

use crate::application::models::user::UserOutput;
use crate::application::services::auth::AuthService;
use crate::application::services::{
    aluno::AlunoService, gestor::GestorService,
    professor::ProfessorService, solicitacao::SolicitacaoService,
    turma::TurmaService,
};
use crate::controllers::auth::AuthController;
use crate::controllers::{
    aluno::AlunoController, gestor::GestorController,
    professor::ProfessorController, solicitacao::SolicitacaoController,
    turma::TurmaController,
};
use crate::infrastructure::database::schemas::auth_schema::Auth;
use crate::infrastructure::database::schemas::solicitacao_schema::SolicitacaoSchema;
use crate::infrastructure::database::schemas::turma_schema::Turma;
use crate::infrastructure::database::schemas::user_schema::OptionUserSchema;
use crate::infrastructure::database::{
    connection::{get_connection, RepoModel},
    schemas::user_schema::UserSchema,
};
use crate::infrastructure::redis::client::init_redis_connection;
use crate::infrastructure::repository::auth_repository::AuthRepository;
use crate::infrastructure::repository::solicitacao_repository::SolicitacaoRepository;
use crate::infrastructure::repository::user_repository::UserRepository;
use crate::infrastructure::repository::{
    aluno_repository::AlunoRepository, gestor_repository::GestorRepository,
    professor_repository::ProfessorRepository, turma_repository::TurmaRepository,
};
use crate::port::query_filter::QueryOptions;
use crate::utils::settings::Env;

#[derive(Clone)]
pub struct Controller {
    pub aluno: AlunoController,
    pub auth: AuthController,
    pub gestor: GestorController,
    pub professor: ProfessorController,
    pub solicitacao: SolicitacaoController,
    pub turma: TurmaController,
}

#[derive(Clone)]
pub struct App {
    pub controllers: Controller,
    pub redis_connection: MultiplexedConnection,
    pub env: Env,
}

pub async fn build(env: &Env) -> App {
    let config = Config::from_url(
        &env.rabbitmq_uri,
        ConfigOptions {
            queue_name: "tcc".to_string(),
            rpc_queue_name: "rpc_tcc".to_string(),
            rpc_exchange_name: "rpc_tcc".to_string(),
        },
    ).expect("Cannot setup rabbitmq config");
    let eventbus = AsyncEventbusRabbitMQ::new(
        config
    ).await;
    let redis_connection = init_redis_connection().await.expect("Cannot connect to Redis");
    let client = get_connection(&env.mongodb_uri)
        .await
        .expect("Cannot connect to MongoDb");
    let db = client.database("teste");
    let user_model = Arc::new(RepoModel::<UserSchema>::new(&db, "users").await);
    let auth_model = RepoModel::<Auth>::new(&db, "auth").await;
    let solicitacao_model = RepoModel::<SolicitacaoSchema>::new(&db, "solicitacoes").await;
    let turma_model = RepoModel::<Turma>::new(&db, "turmas").await;

    let user = UserRepository::new(user_model.clone()).await;
    let aluno = Arc::new(AlunoRepository::new(user_model.clone()).await);
    let auth = AuthRepository::new(auth_model).await;
    let gestor = GestorRepository::new(user_model.clone()).await;
    let professor = ProfessorRepository::new(user_model).await;
    let solicitacao = SolicitacaoRepository::new(solicitacao_model).await;
    let turma = TurmaRepository::new(turma_model).await;

    let aluno_repository = aluno.clone();
    let get_alunos = move |body:Vec<u8>| {
        let aluno_repository = aluno_repository.clone();
        async move {
            let mut query: (OptionUserSchema, QueryOptions) = serde_json::from_slice(&body)?;
            let result = aluno_repository.get_all(query.0.borrow_mut(), query.1).await
                .map( |item| item.into_iter().map(UserOutput::from).collect::<Vec<UserOutput>>())?;

            Ok(serde_json::to_vec(&result)?)
        }
    };

    // Register rpc provider binded with alunos.find
    eventbus.rpc_server(get_alunos, "alunos.find", "application/json", None).await;

    let aluno = AlunoService::new(aluno);
    let auth = AuthService::new(auth, user);
    let gestor = GestorService::new(gestor);
    let professor = ProfessorService::new(professor);
    let solicitacao = SolicitacaoService::new(solicitacao);
    let turma = TurmaService::new(turma);

    App {
        controllers: Controller {
            aluno: AlunoController::new(aluno),
            auth: AuthController::new(auth),
            gestor: GestorController::new(gestor),
            professor: ProfessorController::new(professor),
            solicitacao: SolicitacaoController::new(solicitacao),
            turma: TurmaController::new(turma),
        },
        redis_connection,
        env: env.clone(),
    }
}
