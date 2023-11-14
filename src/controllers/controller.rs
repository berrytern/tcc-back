use async_trait::async_trait;
use actix_web::HttpResponse;

#[async_trait]
pub trait Controller{
    async fn get_one(&self)-> HttpResponse;
    async fn create(&self)-> HttpResponse;
    async fn update_one(&self)-> HttpResponse;
    async fn delete_one(&self)-> HttpResponse;
}