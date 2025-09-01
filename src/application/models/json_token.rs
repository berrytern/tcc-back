use actix_web::{http::header, error::ErrorUnauthorized, Error, FromRequest};
use futures_util::future::{err, ok, Ready};
use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use crate::ENV;


static DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| {
    DecodingKey::from_secret(ENV.jwt_secret.as_bytes())
});
static VALIDATION: Lazy<Validation> = Lazy::new(|| {
    Validation::new(Algorithm::HS256)
});

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonToken{
    pub sub: String,
    pub sub_type: String,
    pub company: String,
    pub scope: String,
    pub exp: u64,
}
impl FromRequest for JsonToken {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let token = req
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_str| auth_str.strip_prefix("Bearer "));
        let Some(token) = token else {
            return err(ErrorUnauthorized("Unauthorized: Missing or malformed token"));
        };

        match decode::<JsonToken>(token, &DECODING_KEY, &VALIDATION){
            Ok(token_data) => ok(token_data.claims),
            Err(_error) => err(ErrorUnauthorized("Unauthorized: Invalid token"))
        }
    }
}