use config::Config;
use dotenv::dotenv;
use serde::Deserialize;
use tokio_postgres::NoTls;

mod db;
mod errors;
mod handlers;
mod models;

use handlers::add_user::*;

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

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .service(actix_web::web::resource("/").to(|| async { "Hello world!" }))
            .service(add_user)
            .app_data(actix_web::web::Data::new(pool.clone()))
    })
    .bind(config.server_addr)?;
    server.run().await
}
