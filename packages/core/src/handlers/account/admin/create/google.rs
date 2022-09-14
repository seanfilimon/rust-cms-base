use actix_web::{get, http::header, post, web, HttpResponse};

use crate::{
    errors::MyError,
    models::Admin,
    prisma::{self, PrismaClient},
    provider::google::{google_callback, login_google, GoogleCreateAccCallback},
    utils::admin_tokens,
};

#[post("/account/admin/create/google")]
pub async fn create_admin_acc_by_google() -> Result<HttpResponse, MyError> {
    let auth_url = login_google("admin");

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, auth_url.to_string()))
        .finish())
}

#[get("/account/admin/create/google/callback")]
pub async fn create_admin_acc_by_google_callback(
    callback: web::Query<GoogleCreateAccCallback>,
    client: web::Data<PrismaClient>,
) -> Result<HttpResponse, MyError> {
    let user: Admin = google_callback(callback.into_inner()).await.into();
    if client
        .admins()
        .find_unique(prisma::admins::email::equals(user.email.clone()))
        .exec()
        .await?
        .is_some()
    {
        return Ok(HttpResponse::Ok().json(admin_tokens(user.into())?));
    }

    let data = client
        .admins()
        .create(
            user.name,
            user.email,
            "".to_string(),
            vec![prisma::admins::provider::set(prisma::Provider::Google)],
        )
        .exec()
        .await?;

    Ok(HttpResponse::Ok().json(admin_tokens(data.into())?))
}
