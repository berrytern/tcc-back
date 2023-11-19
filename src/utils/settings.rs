use derivative::Derivative;
use dotenv::dotenv;
use std::env;

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct Env{
    #[derivative(Default(value = "2"))]
    pub workers: u8,
    pub mongodb_uri: String,
    // #[derivative(Default(value = "false"))]
    // tContinue: bool,
}

pub fn load_env() -> Env {
    let default = Env::default();
    dotenv().ok();
    Env{
        workers: match env::var("WORKERS"){
            Ok(var) => var.parse::<u8>().unwrap(),
            Err(_error) => default.workers,
        },
        mongodb_uri: match env::var("MONGODB_URI"){
            Ok(var) => var,
            Err(_error) => panic!("Environment variable 'WORKERS' not setted")
        }
    }
}