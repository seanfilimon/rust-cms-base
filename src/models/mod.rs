use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;


#[derive(Debug, Deserialize, PostgresMapper, Serialize, PartialEq, Clone)]
#[pg_mapper(table = "users")]
pub struct User {
    #[serde(default)]
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, PostgresMapper, Serialize, PartialEq, Clone)]
#[pg_mapper(table = "admins")]
pub struct Admin {
    #[serde(default)]
    pub name: String,
    pub email: String,
    pub password: String,
}

