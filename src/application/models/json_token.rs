use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonToken{
    pub sub: String,
    pub company: String,
    pub scope: String,
    pub exp: u64,
}