use async_trait::async_trait;
use actix_web::HttpResponse;
use crate::{controllers::controller::Controller};
use crate::port::service::Service;
use crate::port::contructor::Contructor;

pub struct GestorController{
    pub service: dyn Service,
}
impl Contructor for GestorController{
    fn new<Service>(&self, service: &impl Service) ->  Self{
        self.service = service;
        return self;
    }}
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