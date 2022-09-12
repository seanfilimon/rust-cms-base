use actix_web::HttpRequest;

use crate::{
    errors::MyError,
    utils::{validate_admin_token, validate_user_token}, models::{Admin, User},
};

pub(crate) mod user;

pub fn verify_admin_headers(req: &HttpRequest) -> Result<Admin, MyError> {
    let token = req
        .headers()
        .get("Authorization")
        .ok_or(MyError::Unauthorized)?
        .to_str()
        .map_err(|_| MyError::Unauthorized)?
        .replace("Bearer ", "");
    validate_admin_token(&token, "access".as_ref())
}

pub fn verify_user_headers(req: &HttpRequest) -> Result<User, MyError> {
    let token = req
        .headers()
        .get("Authorization")
        .ok_or(MyError::Unauthorized)?
        .to_str()
        .map_err(|_| MyError::Unauthorized)?
        .replace("Bearer ", "");
    validate_user_token(&token, "access".as_ref())
}
