use actix_web::{get, HttpRequest, HttpResponse};

use crate::{errors::MyError, handlers::verify_user_headers};

pub(crate) mod create;
pub(crate) mod login;
pub(crate) mod refresh;

#[get("/account/user")]
pub async fn get_user(req: HttpRequest) -> Result<HttpResponse, MyError> {
    let user = verify_user_headers(&req)?;
    Ok(HttpResponse::Ok().json(user))
}
