use actix_web::{get, HttpRequest, HttpResponse};

use crate::{errors::MyError, handlers::verify_admin_headers};

pub(crate) mod create;
pub(crate) mod login;
pub(crate) mod refresh;

#[get("/account/admin")]
pub async fn get_admin(req: HttpRequest) -> Result<HttpResponse, MyError> {
    let admin = verify_admin_headers(&req)?;
    Ok(HttpResponse::Ok().json(admin))
}
