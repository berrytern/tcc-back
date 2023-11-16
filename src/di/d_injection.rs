use controllers::controller::Controller;
use controllers::gestor_controller::GestorController;

use crate::controllers;

struct Controllers{
    pub gestor: Box<dyn Controller>
}

pub struct App{
    pub controllers: Controllers
}


pub async fn build() -> App{
    App{
        controllers: Controllers{
            gestor: Box::new(GestorController{})
        }
    }
}