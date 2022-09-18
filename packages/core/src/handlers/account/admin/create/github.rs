use std::env;

use actix_web::{get, http::header, post, web, HttpResponse};

use crate::{
    errors::MyError,
    models::Admin,
    prisma::{self, PrismaClient},
    provider::github::{github_callback, login_github, GithubCreateAccCallback},
    utils::admin_tokens,
};

#[post("/account/admin/create/github")]
pub async fn create_admin_acc_by_github() -> Result<HttpResponse, MyError> {
    let auth_url = login_github("admin");

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, auth_url.to_string()))
        .finish())
}

#[get("/account/admin/create/github/callback")]
pub async fn create_admin_acc_by_github_callback(
    callback: web::Query<GithubCreateAccCallback>,
    client: web::Data<PrismaClient>,
) -> Result<HttpResponse, MyError> {
    let user: Admin = github_callback(callback.into_inner()).await.into();
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
            vec![prisma::admins::provider::set(prisma::Provider::Github)],
        )
        .exec()
        .await?;

    Ok(HttpResponse::Ok()
        .append_header((
            header::LOCATION,
            format!("{}/tokens", env::var("CLIENT_URL").unwrap()),
        ))
        .json(admin_tokens(data.into())?))
}
