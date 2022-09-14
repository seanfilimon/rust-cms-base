use actix_web::{get, http::header, post, web, HttpResponse};

use crate::{
    errors::MyError,
    models::User,
    prisma::{self, PrismaClient},
    provider::google::{google_callback, login_google, GoogleCreateAccCallback},
    utils::user_tokens,
};

#[post("/account/user/create/google")]
pub async fn create_user_acc_by_google() -> Result<HttpResponse, MyError> {
    let auth_url = login_google("user");

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, auth_url.to_string()))
        .finish())
}

#[get("/account/user/create/google/callback")]
pub async fn create_user_acc_by_google_callback(
    callback: web::Query<GoogleCreateAccCallback>,
    client: web::Data<PrismaClient>,
) -> Result<HttpResponse, MyError> {
    let user: User = google_callback(callback.into_inner()).await.into();
    if client
        .users()
        .find_unique(prisma::users::email::equals(user.email.clone()))
        .exec()
        .await?
        .is_some()
    {
        return Ok(HttpResponse::Ok().json(user_tokens(user.into())?));
    }

    let data = client
        .users()
        .create(
            user.name,
            user.email,
            "".to_string(),
            vec![prisma::users::provider::set(prisma::Provider::Google)],
        )
        .exec()
        .await?;

    Ok(HttpResponse::Ok().json(user_tokens(data.into())?))
}
