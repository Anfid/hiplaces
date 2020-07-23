use actix_web::{HttpResponse, ResponseError};
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind as DieselDatabaseErrorKind, Error as DieselError},
};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use libreauth::pass::ErrorCode as PassErrorCode;
use serde_derive::Serialize;
use serde_json::json;
use std::io::Error as IoError;
use thiserror::Error;
use validator::ValidationErrors;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {description}")]
    Database {
        kind: DatabaseErrorKind,
        description: String,
    },
    #[error("Validation error")]
    Validation(Vec<ValidationError>),
    #[error("Authorization error: {0}")]
    Authorization(String),
    #[error("{0}")]
    Io(IoError),
    #[allow(unused)]
    #[error("Not Implemented")]
    NotImplemented,
}

#[derive(Debug)]
pub enum DatabaseErrorKind {
    UniqueViolation,
    NotFound,
    Pool,
    Other,
}

#[derive(Debug, Serialize)]
pub struct ValidationError {
    field: String,
    errors: Vec<String>,
}

impl From<PoolError> for Error {
    fn from(e: PoolError) -> Error {
        Error::Database {
            kind: DatabaseErrorKind::Pool,
            description: e.to_string(),
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Error {
        Error::Io(e)
    }
}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(ref kind, ref info) => {
                if let DieselDatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    Error::Database {
                        kind: DatabaseErrorKind::UniqueViolation,
                        description: message,
                    }
                } else {
                    Error::Database {
                        kind: DatabaseErrorKind::Other,
                        description: error.to_string(),
                    }
                }
            }
            DieselError::NotFound => Error::Database {
                kind: DatabaseErrorKind::NotFound,
                description: String::from("Entity not found"),
            },
            _ => Error::Database {
                kind: DatabaseErrorKind::Other,
                description: error.to_string(),
            },
        }
    }
}

impl From<PassErrorCode> for Error {
    fn from(_e: PassErrorCode) -> Self {
        Error::Database {
            kind: DatabaseErrorKind::Other,
            description: String::from("libreauth password error"),
        }
    }
}

impl From<JwtError> for Error {
    fn from(error: JwtError) -> Self {
        match error.kind() {
            JwtErrorKind::InvalidToken => Error::Authorization(String::from("Token is invalid")),
            JwtErrorKind::InvalidIssuer => Error::Authorization(String::from("Issuer is invalid")),
            _ => Error::Authorization(String::from("An issue was found with the token provided")),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Database { kind, description } => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    HttpResponse::Ok().json(json!({ "errors": [description] }))
                }
                DatabaseErrorKind::NotFound => {
                    HttpResponse::Ok().json(json!({ "errors": [description] }))
                }
                _ => HttpResponse::InternalServerError().finish(),
            },
            Error::Io(_) => HttpResponse::InternalServerError().finish(),
            Error::Validation(errs) => HttpResponse::Ok().json(json!({ "errors": errs })),
            Error::Authorization(description) => {
                HttpResponse::Unauthorized().json(json!({ "errors": [description] }))
            }
            Error::NotImplemented => HttpResponse::NotImplemented().finish(),
        }
    }
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Error {
        let e = errors
            .field_errors()
            .iter()
            .map(|(k, v)| ValidationError {
                field: k.to_string(),
                errors: v.iter().map(|e| e.to_string()).collect(),
            })
            .collect();

        Error::Validation(e)
    }
}
