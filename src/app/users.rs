use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use serde_derive::{Deserialize, Serialize};
use std::convert::Into;
use validator::Validate;
use validator_derive::Validate;

use super::AppState;
use crate::auth::{Claims, GenerateJwt, HASHER};
use crate::db::user::UserIdentifier;
use crate::models;
use crate::result::{Error, Result};

// ========== Response types ==========

#[derive(Debug, Serialize)]
struct UserTokenResponse {
    token: String,
    username: String,
    email: String,
}

impl From<models::user::User> for UserTokenResponse {
    fn from(user: models::user::User) -> UserTokenResponse {
        UserTokenResponse {
            token: user.generate_jwt().unwrap(),
            username: user.username,
            email: user.email,
        }
    }
}

#[derive(Debug, Serialize)]
struct UserResponse {
    username: String,
    email: String,
}

impl From<models::user::User> for UserResponse {
    fn from(user: models::user::User) -> UserResponse {
        UserResponse {
            username: user.username,
            email: user.email,
        }
    }
}

// ========== Request types ==========

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUser {
    #[validate(length(
        min = 1,
        max = 20,
        message = "Username length is expected to be between 1 and 20 characters"
    ))]
    pub username: String,
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(
        min = 8,
        max = 128,
        message = "Password length is expected to be between 8 and 128 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateUser {
    #[validate(length(
        min = 1,
        max = 20,
        message = "Username length is expected to be between 1 and 20 characters"
    ))]
    pub username: Option<String>,
    #[validate(email(message = "Invalid email"))]
    pub email: Option<String>,
    #[validate(length(
        min = 8,
        max = 128,
        message = "Password length is expected to be between 8 and 128 characters"
    ))]
    pub password: Option<String>,
}

impl From<RegisterUser> for models::user::NewUser {
    fn from(user: RegisterUser) -> models::user::NewUser {
        models::user::NewUser {
            username: user.username,
            email: user.email,
            password: HASHER.hash(&user.password).unwrap(),
        }
    }
}

// ========== Handlers ==========

pub async fn register(
    state: Data<AppState>,
    Json(user): Json<RegisterUser>,
) -> Result<HttpResponse> {
    user.validate()?;
    state
        .db
        .register_user(user.into())
        .map(|u| HttpResponse::Created().json(UserTokenResponse::from(u)))
}

pub async fn login(state: Data<AppState>, Json(user): Json<LoginUser>) -> Result<HttpResponse> {
    match state.db.login_user(user) {
        Ok(user) => Ok(HttpResponse::Ok().json(UserTokenResponse::from(user))),
        Err(Error::NotFound) => Err(Error::Authorization),
        Err(e) => Err(e),
    }
}

pub async fn update(
    state: Data<AppState>,
    claims: Claims,
    Json(user): Json<UpdateUser>,
) -> Result<HttpResponse> {
    user.validate()?;
    state
        .db
        .update_user(models::user::UpdateUser {
            id: claims.id,
            username: user.username,
            email: user.email,
            password: user.password.map(|p| HASHER.hash(&p).unwrap()),
        })
        .map(|u| HttpResponse::Ok().json(UserResponse::from(u)))
}

pub async fn get_current(state: Data<AppState>, claims: Claims) -> Result<HttpResponse> {
    match state.db.get_user(UserIdentifier::Uuid(claims.id)) {
        Ok(user) => Ok(HttpResponse::Ok().json(UserResponse::from(user))),
        Err(Error::NotFound) => Ok(HttpResponse::Unauthorized().finish()),
        Err(e) => Err(e),
    }
}
