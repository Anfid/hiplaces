use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use serde_derive::{Deserialize, Serialize};
use std::convert::Into;
use validator::Validate;
use validator_derive::Validate;

use super::AppState;
use crate::models;
use crate::result::{Error, Result};
use crate::util::{GenerateJwt, HASHER};

#[derive(Serialize)]
struct UserResponse {
    token: String,
    username: String,
    email: String,
}

impl From<models::user::User> for UserResponse {
    fn from(user: models::user::User) -> UserResponse {
        UserResponse {
            token: user.generate_jwt().unwrap(),
            username: user.username,
            email: user.email,
        }
    }
}

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUser {
    #[validate(length(min = 1, max = 20, message = "invalid username"))]
    pub username: String,
    #[validate(email(message = "invalid email"))]
    pub email: String,
    #[validate(length(min = 8, max = 128, message = "bad password"))]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
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

pub async fn register(state: Data<AppState>, form: Json<RegisterUser>) -> Result<HttpResponse> {
    let user = form.into_inner();
    user.validate()?;
    match state.db.register_user(user.into()) {
        Ok(u) => Ok(HttpResponse::Created().json(UserResponse::from(u))),
        Err(e) => Err(e.into()),
    }
}

pub async fn login(state: Data<AppState>, form: Json<LoginUser>) -> Result<HttpResponse> {
    let user = form.into_inner();
    match state.db.login_user(user) {
        Ok(user) => Ok(HttpResponse::Ok().json(UserResponse::from(user))),
        Err(Error::NotFound) => Err(Error::Authorization),
        Err(e) => Err(e),
    }
}

pub async fn update(_state: Data<AppState>, _req: HttpRequest) -> Result<HttpResponse> {
    Err(Error::NotImplemented)
}

pub async fn get_current(_state: Data<AppState>, _req: HttpRequest) -> Result<HttpResponse> {
    Err(Error::NotImplemented)
}
