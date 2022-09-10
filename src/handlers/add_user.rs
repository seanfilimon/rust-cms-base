use actix_web::{post, web, HttpResponse};
use deadpool_postgres::Pool;

use crate::{db, errors::MyError, models::User};

#[post("/users")]
pub async fn add_user(
    user: web::Json<User>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let user = user.into_inner();
    let client = pool.get().await.map_err(MyError::PoolError)?;
    let user = db::add_user(&client, user).await?;
    Ok(HttpResponse::Ok().json(user))
}
