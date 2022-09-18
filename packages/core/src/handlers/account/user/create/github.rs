use std::env;

use actix_web::{get, http::header, post, web, HttpResponse};

use crate::{
    errors::MyError,
    models::User,
    prisma::{self, PrismaClient},
    provider::github::{github_callback, login_github, GithubCreateAccCallback},
    utils::user_tokens,
};

#[post("/account/user/create/github")]
pub async fn create_user_acc_by_github() -> Result<HttpResponse, MyError> {
    let auth_url = login_github("user");

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, auth_url.to_string()))
        .finish())
}

#[get("/account/user/create/github/callback")]
pub async fn create_user_acc_by_github_callback(
    callback: web::Query<GithubCreateAccCallback>,
    client: web::Data<PrismaClient>,
) -> Result<HttpResponse, MyError> {
    let user: User = github_callback(callback.into_inner()).await.into();
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
            vec![prisma::users::provider::set(prisma::Provider::Github)],
        )
        .exec()
        .await?;

    Ok(HttpResponse::Ok()
        .append_header((
            header::LOCATION,
            format!("{}/tokens", env::var("CLIENT_URL").unwrap()),
        ))
        .json(user_tokens(data.into())?))
}
