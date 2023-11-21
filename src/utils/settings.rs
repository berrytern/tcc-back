use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Env{
    pub workers: u8,
    pub mongodb_uri: String,
    pub jwt_secret: String,
    pub hash_salt: String,
}
impl Default for Env{
    fn default() -> Self {
        Env{
            workers: 2,
            mongodb_uri: "".to_string(),
            jwt_secret: "".to_string(),
            hash_salt: "".to_string(),
        }
    }
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
        },
        jwt_secret: match env::var("JWT_SECRET"){
            Ok(var) => var,
            Err(_error) => panic!("Environment variable 'JWT_SECRET' not setted")
        },
        hash_salt: match env::var("HASH_SALT"){
            Ok(var) => var,
            Err(_error) => panic!("Environment variable 'HASH_SALT' not setted")
        }
    }
}