use crate::infrastructure::database::schemas::user_schema::OptionUser;
use crate::infrastructure::repository::gestor_repository::GestorRepository;
use crate::infrastructure::database::{schemas::user_schema::User,connection::{Model, get_connection}};
use crate::utils::settings::Env;


#[derive(Clone)]
pub struct Repository{
    pub gestor: GestorRepository
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
            gestor: GestorRepository::new(Box::new(user_model)),
        },
    }
}