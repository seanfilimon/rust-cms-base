use actix_web::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};

use crate::errors::MyError;
use crate::handlers::verify_admin_headers;
use crate::models::{Admin, User};
use crate::prisma::{self, PrismaClient};
use crate::utils::{admin_tokens, user_tokens, validate_admin_token, validate_user_token};

use super::verify_user_headers;

#[post("/account/admin/login")]
pub async fn login_admin(
    user: web::Json<Admin>,
    client: web::Data<PrismaClient>,
) -> Result<HttpResponse, MyError> {
    let user = user.into_inner();
    let data = client
        .admins()
        .find_unique(prisma::admins::email::equals(user.email))
        .exec()
        .await?
        .unwrap();
    let verified = verify(user.password, data.password.as_str())?;
    if !verified {
        return Err(MyError::NotFound);
    }
    Ok(HttpResponse::Ok().json(admin_tokens(data.into())?))
}

#[post("/account/admin/create")]
pub async fn create_admin_acc(
    user: web::Json<Admin>,
    client: web::Data<PrismaClient>,
) -> Result<HttpResponse, MyError> {
    let user = user.into_inner();
    if user.name.is_empty() {
        return Err(MyError::BadRequest("Name is required".to_string()));
    }

    let data = client
        .admins()
        .create(
            user.name,
            user.email,
            hash(user.password, DEFAULT_COST)?,
            vec![],
        )
        .exec()
        .await?;

    Ok(HttpResponse::Ok().json(admin_tokens(data.into())?))
}

#[post("/account/user/login")]
pub async fn login_user(
    user: web::Json<User>,
    client: web::Data<PrismaClient>,
) -> Result<HttpResponse, MyError> {
    let user = user.into_inner();
    let data = client
        .users()
        .find_unique(prisma::users::email::equals(user.email))
        .exec()
        .await?
        .unwrap();
    let verified = verify(user.password, data.password.as_str())?;
    if !verified {
        return Err(MyError::NotFound);
    }

    Ok(HttpResponse::Ok().json(user_tokens(data.into())?))
}

#[post("/account/user/create")]
pub async fn create_user_acc(
    user: web::Json<User>,
    client: web::Data<PrismaClient>,
) -> Result<HttpResponse, MyError> {
    let user = user.into_inner();
    if user.name.is_empty() {
        return Err(MyError::BadRequest("Name is required".to_string()));
    }
    let data = client
        .users()
        .create(
            user.name,
            user.email,
            hash(user.password, DEFAULT_COST)?,
            vec![],
        )
        .exec()
        .await?;
    Ok(HttpResponse::Ok().json(user_tokens(data.into())?))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    refresh_token: String,
}

#[post("/account/user/refresh")]
pub async fn refresh_user(refresh: web::Json<RefreshToken>) -> Result<HttpResponse, MyError> {
    let tok = refresh.into_inner().refresh_token;
    let claims = validate_user_token(tok.as_str(), "refresh")?;
    Ok(HttpResponse::Ok().json(user_tokens(claims)?))
}

#[post("/account/admin/refresh")]
pub async fn refresh_admin(refresh: web::Json<RefreshToken>) -> Result<HttpResponse, MyError> {
    let tok = refresh.into_inner().refresh_token;
    let claims = validate_admin_token(tok.as_str(), "refresh")?;
    Ok(HttpResponse::Ok().json(admin_tokens(claims)?))
}

#[get("/account/user")]
pub async fn get_user(req: HttpRequest) -> Result<HttpResponse, MyError> {
    let user = verify_user_headers(&req)?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/account/admin")]
pub async fn get_admin(req: HttpRequest) -> Result<HttpResponse, MyError> {
    let user = verify_admin_headers(&req)?;
    Ok(HttpResponse::Ok().json(user))
}
