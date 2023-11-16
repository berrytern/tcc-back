use async_trait::async_trait;
use actix_web::HttpResponse;
use crate::{controllers::controller::Controller};

pub struct GestorController{
}

#[async_trait]
impl Controller for GestorController {

    async fn get_one(&self) -> HttpResponse {
        HttpResponse::Ok().body("body")
    }
    async fn create(&self) -> HttpResponse {
        HttpResponse::Ok().body("body")
    }
    async fn update_one(&self) -> HttpResponse {
        HttpResponse::Ok().body("body")
    }
    async fn delete_one(&self) -> HttpResponse {
        HttpResponse::Ok().body("body")
    }

}