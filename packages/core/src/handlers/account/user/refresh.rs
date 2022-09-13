use actix_web::{post, web, HttpResponse};

use crate::{
    errors::MyError,
    handlers::RefreshToken,
    utils::{user_tokens, validate_user_token},
};

#[post("/account/user/refresh")]
pub async fn refresh_user(refresh: web::Json<RefreshToken>) -> Result<HttpResponse, MyError> {
    let tok = refresh.into_inner().refresh_token;
    let claims = validate_user_token(tok.as_str(), "refresh")?;
    Ok(HttpResponse::Ok().json(user_tokens(claims)?))
}
