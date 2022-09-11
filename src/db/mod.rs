use bcrypt::{DEFAULT_COST, hash, verify};
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

    if verify(&user.password, &admin.password)? {
        Ok(admin)
    } else {
        Err(MyError::NotFound)
    }
}

pub async fn create_admin_acc(client: &Client, mut user: Admin) -> Result<Admin, MyError> {
    let _stmt = include_str!("../../sql/create_admin.sql");
    let _stmt = _stmt.replace("$table_fields", &Admin::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();
    user.password = hash(&user.password, DEFAULT_COST)?;
    client
        .query(&stmt, &[&user.name, &user.email, &user.password])
        .await?
        .iter()
        .map(|row| Admin::from_row_ref(row).unwrap())
        .collect::<Vec<Admin>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn login_user(client: &Client, user: User) -> Result<User, MyError> {
    let _stmt = include_str!("../../sql/login_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();
    let usr = client
        .query(&stmt, &[&user.email])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound)?;

    if verify(&user.password, &usr.password)? {
        Ok(usr)
    } else {
        Err(MyError::NotFound)
    }
}

pub async fn create_user_acc(client: &Client, mut user: User) -> Result<User, MyError> {
    let _stmt = include_str!("../../sql/create_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();
    user.password = hash(&user.password, DEFAULT_COST)?;
    client
        .query(&stmt, &[&user.name, &user.email, &user.password])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound)
}
