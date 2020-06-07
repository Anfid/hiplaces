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
use crate::result::{HttpError, HttpResult};

#[derive(Serialize)]
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

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUser {
    #[validate(length(min = 1, max = 20, message = "invalid username"))]
    username: String,
    #[validate(email(message = "invalid email"))]
    email: String,
    password: String,
}

impl From<RegisterUser> for models::user::NewUser {
    fn from(user: RegisterUser) -> models::user::NewUser {
        models::user::NewUser {
            username: user.username,
            email: user.email,
            password: user.password,
        }
    }
}

pub async fn register(state: Data<AppState>, form: Json<RegisterUser>) -> HttpResult<HttpResponse> {
    let user = form.into_inner();
    user.validate()?;
    match state.db.register_user(user.into()) {
        Ok(u) => Ok(HttpResponse::Created().json(UserResponse::from(u))),
        Err(e) => Err(e.into()),
    }
}

pub async fn login(_state: Data<AppState>, _req: HttpRequest) -> HttpResult<HttpResponse> {
    Err(HttpError::NotImplemented)
}

pub async fn update(_state: Data<AppState>, _req: HttpRequest) -> HttpResult<HttpResponse> {
    Err(HttpError::NotImplemented)
}

pub async fn get_current(_state: Data<AppState>, _req: HttpRequest) -> HttpResult<HttpResponse> {
    Err(HttpError::NotImplemented)
}
