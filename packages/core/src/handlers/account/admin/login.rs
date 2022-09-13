use actix_web::{post, web, HttpResponse};
use bcrypt::verify;

use crate::{
    errors::MyError,
    models::Admin,
    prisma::{self, PrismaClient},
    utils::admin_tokens,
};

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
