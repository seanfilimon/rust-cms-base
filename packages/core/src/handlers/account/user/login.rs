use actix_web::{post, web, HttpResponse};
use bcrypt::verify;

use crate::{
    errors::MyError,
    models::User,
    prisma::{self, PrismaClient},
    utils::user_tokens,
};

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
