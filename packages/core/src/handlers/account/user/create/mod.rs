use actix_web::{post, web, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};

use crate::{errors::MyError, models::User, prisma::PrismaClient, utils::user_tokens};

pub(crate) mod github;
pub(crate) mod google;

#[post("/account/user/create")]
pub async fn create_user_acc(
    user: web::Json<User>,
    client: web::Data<PrismaClient>,
) -> Result<HttpResponse, MyError> {
    let user = user.into_inner();
    if user.name.is_empty() {
        return Err(MyError::BadRequest("Name is required".to_string()));
    }
    let data = client
        .users()
        .create(
            user.name,
            user.email,
            hash(user.password, DEFAULT_COST)?,
            vec![],
        )
        .exec()
        .await?;
    Ok(HttpResponse::Ok().json(user_tokens(data.into())?))
}
