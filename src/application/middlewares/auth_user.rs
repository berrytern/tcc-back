use std::pin::Pin;

use actix_web::{error::ErrorUnauthorized, Error, FromRequest};
use futures::Future;

use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    aud: String,         // Optional. Audience
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    sub: String,         // Optional. Subject (whom token refers to)
}
impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<AuthUser, Error>>>>;
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        if let Some(hAuth) = req.headers().get("Authorization") {
            if hAuth.len() > 7 && hAuth.to_str()?.get(0..5) == Some("token") {
                let token = hAuth.to_str()?.get(5..).unwrap();
                let tk_data = decode::<AuthUser>(token,&DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::EdDSA))?;
                return Box::pin(async move {
                    return Ok(tk_data.claims);
                });
            }
        }
        Box::pin(async { Err(ErrorUnauthorized("unauthorized")) })
    }
}
