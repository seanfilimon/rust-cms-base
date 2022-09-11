use actix_web::{post, web, HttpResponse};
use deadpool_postgres::Pool;
use serde::{Serialize, Deserialize};

use crate::{db, errors::MyError, models::User};
use crate::models::Admin;
use crate::utils::{admin_tokens, user_tokens, validate_user_token, validate_admin_token};

#[post("/account/admin/login")]
pub async fn login_admin(
    user: web::Json<Admin>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let user = user.into_inner();
    let client = pool.get().await.map_err(MyError::PoolError)?;
    let user = db::login_admin(&client, user).await?;
    Ok(HttpResponse::Ok().json(admin_tokens(user)?))
}

#[post("/account/admin/create")]
pub async fn create_admin_acc(
    user: web::Json<Admin>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let user = user.into_inner();
    if user.name.is_empty() {
        return Err(MyError::BadRequest("Name is required".to_string()));
    }
    let client = pool.get().await.map_err(MyError::PoolError)?;
    let user = db::create_admin_acc(&client, user).await?;
    Ok(HttpResponse::Ok().json(admin_tokens(user)?))
}

#[post("/account/user/login")]
pub async fn login_user(
    user: web::Json<User>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let user = user.into_inner();
    let client = pool.get().await.map_err(MyError::PoolError)?;
    let user = db::login_user(&client, user).await?;
    Ok(HttpResponse::Ok().json(user_tokens(user)?))
}

#[post("/account/user/create")]
pub async fn create_user_acc(
    user: web::Json<User>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let user = user.into_inner();
    if user.name.is_empty() {
        return Err(MyError::BadRequest("Name is required".to_string()));
    }
    let client = pool.get().await.map_err(MyError::PoolError)?;
    let user = db::create_user_acc(&client, user).await?;
    Ok(HttpResponse::Ok().json(user_tokens(user)?))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    refresh_token: String,
}

#[post("/account/user/refresh")]
pub async fn refresh_user(
    refresh: web::Json<RefreshToken>,
) -> Result<HttpResponse, MyError> {
    let tok = refresh.into_inner().refresh_token;
    let claims = validate_user_token(tok.as_str(), "refresh")?;
    Ok(HttpResponse::Ok().json(user_tokens(claims)?))
}

#[post("/account/admin/refresh")]
pub async fn refresh_admin(
    refresh: web::Json<RefreshToken>,
) -> Result<HttpResponse, MyError> {
    let tok = refresh.into_inner().refresh_token;
    let claims = validate_admin_token(tok.as_str(), "refresh")?;
    Ok(HttpResponse::Ok().json(admin_tokens(claims)?))
}
