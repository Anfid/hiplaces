use actix_web::{HttpResponse, ResponseError};
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind as DieselDatabaseErrorKind, Error as DieselError},
};
use serde_json::{json, Map as JsonMap, Value as JsonValue};
use std::io::Error as IoError;
use thiserror::Error;
use validator::ValidationErrors;

pub type Result<T> = std::result::Result<T, Error>;
pub type HttpResult<T> = std::result::Result<T, HttpError>;

#[derive(Debug)]
pub enum DatabaseErrorKind {
    UniqueViolation,
    NotFound,
    Pool,
    Other,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {description}")]
    Database {
        kind: DatabaseErrorKind,
        description: String,
    },
    #[error("{0}")]
    Io(IoError),
    #[error("Not Implemented")]
    NotImplemented,
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

impl From<Error> for HttpError {
    fn from(e: Error) -> HttpError {
        match e {
            Error::Database { kind, description } => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    HttpError::UnprocessableEntity(json!({ "error": description }))
                }
                DatabaseErrorKind::NotFound => HttpError::NotFound,
                _ => HttpError::InternalServerError,
            },
            Error::Io(_) => HttpError::InternalServerError,
            Error::NotImplemented => HttpError::NotImplemented,
        }
    }
}

#[derive(Debug, Error)]
pub enum HttpError {
    //404
    #[error("Not Found")]
    NotFound,
    // 422
    #[error("Unprocessable Entity: {0}")]
    UnprocessableEntity(JsonValue),
    // 500
    #[error("Internal Server Error")]
    InternalServerError,
    // 501
    #[error("Not Implemented")]
    NotImplemented,
}

impl HttpError {
    fn to_json(&self) -> JsonValue {
        match self {
            HttpError::UnprocessableEntity(json) => json.clone(),
            _ => {
                let message = self.to_string();
                json!({ "error": message })
            }
        }
    }
}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        match self {
            HttpError::NotFound => HttpResponse::NotFound().json(self.to_json()),
            HttpError::UnprocessableEntity(_) => {
                HttpResponse::UnprocessableEntity().json(self.to_json())
            }
            HttpError::InternalServerError => {
                HttpResponse::InternalServerError().finish()
            }
            HttpError::NotImplemented => HttpResponse::NotImplemented().json(self.to_json()),
        }
    }
}

impl From<ValidationErrors> for HttpError {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();

        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<JsonValue> = errors.iter().map(|error| json!(error.message)).collect();
            err_map.insert(field.to_string(), json!(errors));
        }

        HttpError::UnprocessableEntity(json!({
            "errors": err_map,
        }))
    }
}
