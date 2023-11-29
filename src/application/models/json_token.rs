use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonToken{
    pub sub: String,
    pub sub_type: String,
    pub company: String,
    pub scope: String,
    pub exp: u64,
}