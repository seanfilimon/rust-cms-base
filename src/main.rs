use actix_web::web;
use config::Config;
use dotenv::dotenv;
use serde::Deserialize;
use tokio_postgres::NoTls;

mod db;
mod errors;
mod handlers;
mod models;
mod utils;
mod ws;

use handlers::user::*;
use ws::*;

#[derive(Debug, Default, Deserialize)]
pub struct MyConfig {
    pub server_addr: String,
    pub pg: deadpool_postgres::Config,
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

    let pool = config.pg.create_pool(None, NoTls).unwrap();
    let srvmon = ServerMonitor::new();

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(srvmon.clone()))
            .service(actix_web::web::resource("/").to(|| async { "Hello world!" }))
            .service(actix_web::web::resource("/ws/").route(web::get().to(ws_index)))
            .service(login_admin)
            .service(create_admin_acc)
            .service(login_user)
            .service(create_user_acc)
            .service(refresh_user)
            .service(refresh_admin)
            .service(get_user)
            .service(get_admin)
    })
    .bind(config.server_addr)?;
    server.run().await
}
