use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
#[derive(utoipa::ToSchema)]
pub struct AccessToken {
    pub access_token: String,
    pub refresh_token: String,
}