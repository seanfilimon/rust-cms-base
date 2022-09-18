pub(crate) mod github;
pub(crate) mod google;

use actix_web::{post, web, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};

use crate::{errors::MyError, models::Admin, prisma::PrismaClient, utils::admin_tokens};

#[post("/account/admin/create")]
pub async fn create_admin_acc(
    user: web::Json<Admin>,
    client: web::Data<PrismaClient>,
) -> Result<HttpResponse, MyError> {
    let user = user.into_inner();
    if user.name.is_empty() {
        return Err(MyError::BadRequest("Name is required".to_string()));
    }

    let data = client
        .admins()
        .create(
            user.name,
            user.email,
            hash(user.password, DEFAULT_COST)?,
            vec![],
        )
        .exec()
        .await?;

    Ok(HttpResponse::Ok().json(admin_tokens(data.into())?))
}
