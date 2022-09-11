use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::MyError, models::User};
use crate::models::Admin;


pub async fn login_admin(client: &Client, user: Admin) -> Result<Admin, MyError> {
    let _stmt = include_str!("../../sql/login_admin.sql");
    let _stmt = _stmt.replace("$table_fields", &Admin::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();
    let admin = client
        .query(&stmt, &[&user.email])
        .await?
        .iter()
        .map(|row| Admin::from_row_ref(row).unwrap())
        .collect::<Vec<Admin>>()
        .pop()
        .ok_or(MyError::NotFound)?;

    if admin.password == user.password {
        Ok(admin)
    } else {
        Err(MyError::NotFound)
    }
}