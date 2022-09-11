use actix_web::{post, web, HttpResponse};
use deadpool_postgres::Pool;

use crate::{db, errors::MyError, models::User};
use crate::models::Admin;
use crate::utils::{tokens, validate_token};

#[post("/account/admin/login")]
pub async fn login_admin(
    user: web::Json<Admin>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let user = user.into_inner();
    let client = pool.get().await.map_err(MyError::PoolError)?;
    let user = db::login_admin(&client, user).await?;
    Ok(HttpResponse::Ok().json(tokens(user)))
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
    Ok(HttpResponse::Ok().json(tokens(user)))
}

#[post("/account/user/login")]
pub async fn login_user(
    user: web::Json<User>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let user = user.into_inner();
    let client = pool.get().await.map_err(MyError::PoolError)?;
    let user = db::login_user(&client, user).await?;
    Ok(HttpResponse::Ok().json(tokens(user)))
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
    Ok(HttpResponse::Ok().json(tokens(user)))
}

struct RefreshToken {
    refresh_token: String,
}

#[post("/account/user/refresh")]
pub async fn refresh(
    refresh: web::Json<RefreshToken>,
) -> Result<HttpResponse, MyError> {
    let refresh = refresh.into_inner().refresh_token;
    let claims = validate_token::<User>(&refresh, "refresh")?;
    Ok(HttpResponse::Ok().json(tokens(claims)))
}

#[post("/account/admin/refresh")]
pub async fn refresh_admin(
    refresh: web::Json<RefreshToken>,
) -> Result<HttpResponse, MyError> {
    let refresh = refresh.into_inner().refresh_token;
    let claims = validate_token::<Admin>(&refresh, "refresh")?;
    Ok(HttpResponse::Ok().json(tokens(claims)))
}
