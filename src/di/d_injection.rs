use crate::controllers::{
    aluno_controller::AlunoController, gestor_controller::GestorController,
    professor_controller::ProfessorController, solicitacao_controller::SolicitacaoController,
    turma_controller::TurmaController,
};
use crate::infrastructure::database::schemas::solicitacao_schema::Solicitacao;
use crate::infrastructure::database::schemas::turma_schema::Turma;
use crate::infrastructure::database::{
    connection::{get_connection, Model},
    schemas::user_schema::User,
};
use crate::infrastructure::repository::solicitacao_repository::SolicitacaoRepository;
use crate::infrastructure::repository::{
    aluno_repository::AlunoRepository, gestor_repository::GestorRepository,
    professor_repository::ProfessorRepository, turma_repository::TurmaRepository,
};
use crate::utils::settings::Env;

#[derive(Clone)]
pub struct Repository {
    pub solicitacao: SolicitacaoRepository,
    pub turma: TurmaRepository,
}
#[derive(Clone)]
pub struct Controller {
    pub aluno: AlunoController,
    pub gestor: GestorController,
    pub professor: ProfessorController,
    pub solicitacao: SolicitacaoController,
    pub turma: TurmaController,
}

#[derive(Clone)]
pub struct App {
    pub controllers: Controller,
}

pub async fn build(env: &Env) -> App {
    let client = get_connection(&env.mongodb_uri)
        .await
        .expect("Cannot connect to MongoDb");
    let db = client.database("teste");
    let user_model = Model::<User>::new(&db, "users").await;
    let solicitacao_model = Model::<Solicitacao>::new(&db, "solicitacoes").await;
    let turma_model = Model::<Turma>::new(&db, "turmas").await;

    let aluno = AlunoRepository::new(Box::new(user_model.clone())).await;
    let gestor = GestorRepository::new(Box::new(user_model.clone())).await;
    let professor = ProfessorRepository::new(Box::new(user_model.clone())).await;
    let solicitacao = SolicitacaoRepository::new(Box::new(solicitacao_model.clone())).await;
    let turma = TurmaRepository::new(Box::new(turma_model.clone())).await;

    App {
        controllers: Controller {
            aluno: AlunoController::new(Box::new(aluno)),
            gestor: GestorController::new(Box::new(gestor)),
            professor: ProfessorController::new(Box::new(professor)),
            solicitacao: SolicitacaoController::new(Box::new(solicitacao)),
            turma: TurmaController::new(Box::new(turma)),
        },
    }
}
