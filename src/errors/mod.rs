use std::fmt::{Formatter};
use actix_web::{HttpResponse, ResponseError};
use actix_web::cookie::time::error::ComponentRange;
use bcrypt::BcryptError;
use derive_more::{From};

#[derive(From, Debug)]
pub enum MyError {
    NotFound,
    Unauthorized,
    PGError(tokio_postgres::Error),
    PGMError(tokio_pg_mapper::Error),
    PoolError(deadpool_postgres::PoolError),
    BadRequest(String),
}

impl std::error::Error for MyError {}


impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::NotFound => write!(f, "Not found"),
            MyError::Unauthorized => write!(f, "Unauthorized"),
            MyError::PGError(e) => write!(f, "Postgres error: {}", e),
            MyError::PGMError(e) => write!(f, "Postgres mapper error: {}", e),
            MyError::PoolError(e) => write!(f, "Pool error: {}", e),
            MyError::BadRequest(e) => write!(f, "Bad request: {}", e),
        }
    }
}

impl From<BcryptError> for MyError {
    fn from(e: BcryptError) -> Self {
        MyError::BadRequest(format!("Bcrypt error: {}", e))
    }
}

impl From<ComponentRange> for MyError {
    fn from(e: ComponentRange) -> Self {
        MyError::BadRequest(format!("ComponentRange error: {}", e))
    }
}

impl From<jsonwebtoken::errors::Error> for MyError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        MyError::BadRequest(format!("JWT error: {}", e.to_string()))
    }
}

impl From<actix_web::Error> for MyError {
    fn from(e: actix_web::error::Error) -> Self {
        MyError::BadRequest(format!("Actix error: {}", e.to_string()))
    }
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            MyError::BadRequest(ref err) => HttpResponse::BadRequest().body(err.to_string()),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
