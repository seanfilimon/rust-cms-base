use actix_web::HttpRequest;
use serde::{Deserialize, Serialize};

use crate::{
    errors::MyError,
    models::{Admin, User},
    utils::{validate_admin_token, validate_user_token},
};

pub(crate) mod account {
    pub(crate) mod admin;
    pub(crate) mod user;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    refresh_token: String,
}

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
