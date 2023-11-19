use crate::infrastructure::database::schemas::user_schema::OptionUser;
use crate::infrastructure::repository::{aluno_repository::AlunoRepository,gestor_repository::GestorRepository,professor_repository::ProfessorRepository};
use crate::infrastructure::database::{schemas::user_schema::User,connection::{Model, get_connection}};
use crate::utils::settings::Env;


#[derive(Clone)]
pub struct Repository{
    pub aluno: AlunoRepository,
    pub gestor: GestorRepository,
    pub professor: ProfessorRepository
}

#[derive(Clone)]
pub struct App {
    pub repositories: Repository
}


pub async fn build(env: &Env) -> App{
    let client = get_connection(&env.mongodb_uri).await.ok().expect("Cannot connect to MongoDb");
    let db = client.database("teste");
    let user_model = Model::<User>::new(db, "users").await;
   
    App{
        repositories: Repository{
            aluno: AlunoRepository::new(Box::new(user_model.clone())),
            gestor: GestorRepository::new(Box::new(user_model.clone())),
            professor: ProfessorRepository::new(Box::new(user_model.clone())),
        },
    }
}