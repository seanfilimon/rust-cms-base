use serde::{Deserialize, Serialize};

use crate::prisma;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Admin {
    #[serde(default)]
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub roles: Vec<Roles>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    #[serde(default)]
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub roles: Vec<Roles>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Roles {
    pub name: String,
    pub permissions: Vec<String>,
}

impl From<prisma::roles::Data> for Roles {
    fn from(role: prisma::roles::Data) -> Self {
        Self {
            name: role.name,
            permissions: role.permissions.iter().map(|p| p.to_string()).collect(),
        }
    }
}

impl From<prisma::users::Data> for User {
    fn from(user: prisma::users::Data) -> Self {
        Self {
            name: user.name,
            email: user.email,
            password: user.password,
            roles: user
                .roles
                .into_iter()
                .flatten()
                .map(|r| Roles::from(r))
                .collect::<Vec<Roles>>(),
        }
    }
}

impl From<prisma::admins::Data> for Admin {
    fn from(admin: prisma::admins::Data) -> Self {
        Self {
            name: admin.name,
            email: admin.email,
            password: admin.password,
            roles: admin
                .roles
                .into_iter()
                .flatten()
                .map(|r| Roles::from(r))
                .collect::<Vec<Roles>>(),
        }
    }
}
