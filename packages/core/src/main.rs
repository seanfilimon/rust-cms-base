use std::sync::Arc;

use actix_web::{dev::ServiceRequest, web, Error};
use actix_web_grants::GrantsMiddleware;
use config::Config;
use dotenv::dotenv;
use errors::MyError;
use serde::Deserialize;

mod errors;
mod handlers;
mod models;
mod prisma;
mod utils;
mod ws;

use handlers::account::*;
use utils::{validate_admin_token, validate_user_token};
use ws::*;

#[derive(Debug, Default, Deserialize)]
pub struct MyConfig {
    pub server_addr: String,
}

async fn extract(req: &ServiceRequest) -> Result<Vec<String>, Error> {
    let headers = req
        .headers()
        .get("Authorization")
        .ok_or(MyError::Unauthorized)?
        .to_str()
        .map_err(|_| MyError::Unauthorized)?
        .replace("Bearer ", "");

    let admin = validate_admin_token(headers.as_str(), "access");
    if let Ok(admin) = admin {
        return Ok(admin
            .roles
            .iter()
            .map(|r| r.permissions.clone())
            .flatten()
            .collect());
    } else {
        let user = validate_user_token(headers.as_str(), "access");
        if let Ok(user) = user {
            return Ok(user
                .roles
                .iter()
                .map(|r| r.permissions.clone())
                .flatten()
                .collect());
        }
    }

    Ok(vec![])
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = Config::builder()
        .add_source(config::Environment::default())
        .build()
        .unwrap()
        .try_deserialize::<MyConfig>()
        .unwrap();

    let client = Arc::new(prisma::new_client().await.unwrap());

    let srvmon = ServerMonitor::new();

    let server = actix_web::HttpServer::new(move || {
        let roles = GrantsMiddleware::with_extractor(extract);
        actix_web::App::new()
            .wrap(roles)
            .app_data(actix_web::web::Data::new(client.clone()))
            .app_data(actix_web::web::Data::new(srvmon.clone()))
            .service(actix_web::web::resource("/").to(|| async { "Hello world!" }))
            .service(actix_web::web::resource("/ws/").route(web::get().to(ws_index)))
            .service(admin::login::login_admin)
            .service(admin::create::create_admin_acc)
            .service(user::login::login_user)
            .service(user::create::create_user_acc)
            .service(user::refresh::refresh_user)
            .service(admin::refresh::refresh_admin)
            .service(user::get_user)
            .service(admin::get_admin)
    })
    .bind(config.server_addr)?;
    server.run().await
}
