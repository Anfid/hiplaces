use actix_web::{HttpResponse, ResponseError};
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind as DieselDatabaseErrorKind, Error as DieselError},
};
use jsonwebtoken::errors::Error as JwtError;
use libreauth::pass::ErrorCode as PassErrorCode;
use serde_derive::Serialize;
use serde_json::{json, Value as JsonValue};
use std::io::Error as IoError;
use thiserror::Error;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "kind", content = "info")]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[error("Entity already exists")]
    AlreadyExists,
    #[error("Entity not found")]
    NotFound,
    #[error("Validation error")]
    FieldValidation(HashMap<&'static str, Vec<ValidationError>>),
    #[error("Authorization error")]
    Authorization,
    #[error("Internal server error")]
    Internal,
    #[allow(unused)]
    #[error("Not Implemented")]
    NotImplemented,
}

impl Error {
    fn to_json(&self) -> JsonValue {
        json!({ "error": self })
    }
}

impl From<PoolError> for Error {
    fn from(e: PoolError) -> Error {
        error!("{}", e);
        Error::Internal
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Error {
        error!("{}", e);
        Error::Internal
    }
}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => Error::NotFound,
            DieselError::DatabaseError(DieselDatabaseErrorKind::UniqueViolation, _) => {
                Error::AlreadyExists
            }
            _ => {
                error!("{}", error);
                Error::Internal
            }
        }
    }
}

impl From<PassErrorCode> for Error {
    fn from(e: PassErrorCode) -> Self {
        error!("libreauth: {:?}", e);
        Error::Internal
    }
}

// TODO: Add specific error variants if needed
impl From<JwtError> for Error {
    fn from(error: JwtError) -> Self {
        info!("Jwt error: {}", error);
        Error::Authorization
    }
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Error {
        Error::FieldValidation(
            errors
                .into_errors()
                .into_iter()
                .filter_map(|(k, v)| {
                    if let ValidationErrorsKind::Field(errors) = v {
                        Some((k, errors))
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::AlreadyExists => HttpResponse::Ok().json(self.to_json()),
            Error::NotFound => HttpResponse::Ok().json(self.to_json()),
            Error::FieldValidation(_) => HttpResponse::Ok().json(self.to_json()),
            Error::Authorization => HttpResponse::Ok().json(self.to_json()),
            Error::Internal => HttpResponse::InternalServerError().finish(),
            Error::NotImplemented => HttpResponse::NotImplemented().finish(),
        }
    }
}
