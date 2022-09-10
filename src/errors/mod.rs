use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};

#[derive(Display, From, Debug)]
pub enum MyError {
    NotFound,
    PGError(tokio_postgres::Error),
    PGMError(tokio_pg_mapper::Error),
    PoolError(deadpool_postgres::PoolError),
}

impl std::error::Error for MyError {}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
