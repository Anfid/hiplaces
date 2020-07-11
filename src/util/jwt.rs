use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

use crate::models::user::User;
use crate::result::Result;

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
        let exp = (Utc::now() + Duration::days(21)).timestamp();
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
