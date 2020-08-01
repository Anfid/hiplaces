use actix_web::http::header::AUTHORIZATION;
use actix_web::{dev::Payload, FromRequest, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use libreauth::pass::{Algorithm, HashBuilder, Hasher};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

use crate::models::user::User;
use crate::result::Result;

pub const PWD_ALGORITHM: Algorithm = Algorithm::Argon2;
pub const PWD_SCHEME_VERSION: usize = 1;

lazy_static! {
    pub static ref HASHER: Hasher = {
        HashBuilder::new()
            .algorithm(PWD_ALGORITHM)
            .version(PWD_SCHEME_VERSION)
            .finalize()
            .unwrap()
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: Uuid,
    pub exp: i64,
}

pub trait GenerateJwt {
    fn generate_jwt(&self) -> Result<String>;
}

impl GenerateJwt for User {
    fn generate_jwt(&self) -> Result<String> {
        let exp = (Utc::now() + Duration::days(1)).timestamp();
        let claims = Claims { id: self.id, exp };

        let header = Header::default();
        let secret = &get_secret();
        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        Ok(token)
    }
}

pub trait DecodeJwt {
    fn decode_jwt(&self) -> Result<TokenData<Claims>>;
}

impl DecodeJwt for &str {
    fn decode_jwt(&self) -> Result<TokenData<Claims>> {
        decode::<Claims>(
            &self,
            &DecodingKey::from_secret(get_secret().as_bytes()),
            &Validation::default(),
        )
        .map_err(Into::into)
    }
}

fn get_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| String::from("secret"))
}

impl FromRequest for Claims {
    type Error = HttpResponse;
    type Future = Ready<std::result::Result<Self, HttpResponse>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|auth| auth.to_str().ok())
            .and_then(|id| id.decode_jwt().ok());

        if let Some(token_data) = auth {
            ok(token_data.claims)
        } else {
            err(HttpResponse::Unauthorized().finish())
        }
    }
}
