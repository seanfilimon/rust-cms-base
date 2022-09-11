use actix_web::{post, web, HttpResponse};
use deadpool_postgres::Pool;

use crate::{db, errors::MyError, models::User};
use crate::models::Admin;
use crate::utils::tokens;

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