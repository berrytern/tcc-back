use crate::infrastructure::database::schemas::solicitacao_schema::Solicitacao;
use crate::infrastructure::database::schemas::turma_schema::Turma;
use crate::infrastructure::repository::solicitacao_repository::SolicitacaoRepository;
use crate::infrastructure::repository::{
    aluno_repository::AlunoRepository,
    gestor_repository::GestorRepository,
    professor_repository::ProfessorRepository,
    turma_repository::TurmaRepository};
use crate::infrastructure::database::{schemas::user_schema::User,connection::{Model, get_connection}};
use crate::utils::settings::Env;


#[derive(Clone)]
pub struct Repository{
    pub aluno: AlunoRepository,
    pub gestor: GestorRepository,
    pub professor: ProfessorRepository,
    pub solicitacao: SolicitacaoRepository,
    pub turma: TurmaRepository
}

#[derive(Clone)]
pub struct App {
    pub repositories: Repository
}


pub async fn build(env: &Env) -> App{
    let client = get_connection(&env.mongodb_uri).await.expect("Cannot connect to MongoDb");
    let db = client.database("teste");
    let user_model = Model::<User>::new(&db, "users").await;
    let solicitacao_model = Model::<Solicitacao>::new(&db, "solicitacoes").await;
    let turma_model = Model::<Turma>::new(&db, "turmas").await;
   
    App{
        repositories: Repository{
            aluno: AlunoRepository::new(Box::new(user_model.clone())).await,
            gestor: GestorRepository::new(Box::new(user_model.clone())).await,
            professor: ProfessorRepository::new(Box::new(user_model.clone())).await,
            solicitacao: SolicitacaoRepository::new(Box::new(solicitacao_model.clone())).await,
            turma: TurmaRepository::new(Box::new(turma_model.clone())).await,
        },
    }
}