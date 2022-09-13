use actix_web::{post, web, HttpResponse};

use crate::{
    errors::MyError,
    handlers::RefreshToken,
    utils::{admin_tokens, validate_admin_token},
};

#[post("/account/admin/refresh")]
pub async fn refresh_admin(refresh: web::Json<RefreshToken>) -> Result<HttpResponse, MyError> {
    let tok = refresh.into_inner().refresh_token;
    let claims = validate_admin_token(tok.as_str(), "refresh")?;
    Ok(HttpResponse::Ok().json(admin_tokens(claims)?))
}
