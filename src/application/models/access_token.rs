use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub refresh_token: String,
}