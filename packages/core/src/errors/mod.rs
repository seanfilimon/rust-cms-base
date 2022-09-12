use actix_web::cookie::time::error::ComponentRange;
use actix_web::{HttpResponse, ResponseError};
use bcrypt::BcryptError;
use prisma_client_rust::QueryError;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum MyError {
    NotFound,
    Unauthorized,
    QueryError(String),
    BadRequest(String),
}

impl std::error::Error for MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::NotFound => write!(f, "Not found"),
            MyError::Unauthorized => write!(f, "Unauthorized"),
            MyError::QueryError(e) => write!(f, "Query error: {}", e),
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

impl From<QueryError> for MyError {
    fn from(e: QueryError) -> Self {
        MyError::QueryError(e.to_string())
    }
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::BadRequest(ref err) => HttpResponse::BadRequest().body(err.to_string()),
            MyError::Unauthorized => HttpResponse::Unauthorized().finish(),
            MyError::QueryError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }
}
