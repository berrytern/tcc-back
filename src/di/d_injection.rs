use crate::application::services::auth_service::AuthService;
use crate::application::services::{
    aluno_service::AlunoService, gestor_service::GestorService,
    professor_service::ProfessorService, solicitacao_service::SolicitacaoService,
    turma_service::TurmaService,
};
use crate::controllers::auth_controller::AuthController;
use crate::controllers::{
    aluno_controller::AlunoController, gestor_controller::GestorController,
    professor_controller::ProfessorController, solicitacao_controller::SolicitacaoController,
    turma_controller::TurmaController,
};
use crate::infrastructure::database::schemas::auth_schema::Auth;
use crate::infrastructure::database::schemas::solicitacao_schema::Solicitacao;
use crate::infrastructure::database::schemas::turma_schema::Turma;
use crate::infrastructure::database::{
    connection::{get_connection, Model},
    schemas::user_schema::User,
};
use crate::infrastructure::repository::auth_repository::AuthRepository;
use crate::infrastructure::repository::solicitacao_repository::SolicitacaoRepository;
use crate::infrastructure::repository::user_repository::UserRepository;
use crate::infrastructure::repository::{
    aluno_repository::AlunoRepository, gestor_repository::GestorRepository,
    professor_repository::ProfessorRepository, turma_repository::TurmaRepository,
};
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
    pub env: Env,
}

pub async fn build(env: &Env) -> App {
    let client = get_connection(&env.mongodb_uri)
        .await
        .expect("Cannot connect to MongoDb");
    let db = client.database("teste");
    let user_model = Model::<User>::new(&db, "users").await;
    let auth_model = Model::<Auth>::new(&db, "auth").await;
    let solicitacao_model = Model::<Solicitacao>::new(&db, "solicitacoes").await;
    let turma_model = Model::<Turma>::new(&db, "turmas").await;

    let user = UserRepository::new(Box::new(user_model.clone())).await;
    let aluno = AlunoRepository::new(Box::new(user_model.clone())).await;
    let auth = AuthRepository::new(Box::new(auth_model)).await;
    let gestor = GestorRepository::new(Box::new(user_model.clone())).await;
    let professor = ProfessorRepository::new(Box::new(user_model)).await;
    let solicitacao = SolicitacaoRepository::new(Box::new(solicitacao_model)).await;
    let turma = TurmaRepository::new(Box::new(turma_model)).await;

    let aluno = AlunoService::new(Box::new(aluno));
    let auth = AuthService::new(Box::new(auth), Box::new(user));
    let gestor = GestorService::new(Box::new(gestor));
    let professor = ProfessorService::new(Box::new(professor));
    let solicitacao = SolicitacaoService::new(Box::new(solicitacao));
    let turma = TurmaService::new(Box::new(turma));

    App {
        controllers: Controller {
            aluno: AlunoController::new(Box::new(aluno)),
            auth: AuthController::new(Box::new(auth)),
            gestor: GestorController::new(Box::new(gestor)),
            professor: ProfessorController::new(Box::new(professor)),
            solicitacao: SolicitacaoController::new(Box::new(solicitacao)),
            turma: TurmaController::new(Box::new(turma)),
        },
        env: env.clone(),
    }
}
