use controllers::controller::Controller;
use controllers::gestor_controller::GestorController;

use crate::controllers;

struct Controllers{
    pub gestor: Box<dyn Controller>
}

struct App{
    pub controllers: Controllers
}


pub async fn build() -> App{
    let a = GestorController{
        service: todo!(),
    };
    let gestor_controller = Box::new(a);

    App{
        controllers: Controllers{
            gestor: Box::new(a)
        }
    }
}