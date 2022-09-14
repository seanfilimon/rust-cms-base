use actix_web::{http::header, post, get, web, HttpResponse};

use crate::{
    errors::MyError,
    models::Admin,
    prisma::{self, PrismaClient},
    provider::google::{create_acc, google_callback, GoogleCreateAccCallback},
};

#[post("/account/admin/create/google")]
pub async fn create_admin_acc_by_google() -> Result<HttpResponse, MyError> {
    let auth_url = create_acc("admin");

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

    Ok(HttpResponse::Ok().json(data))
}
