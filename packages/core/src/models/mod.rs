use serde::{Deserialize, Serialize};

use crate::{
    prisma::{self, Provider},
    provider::{github::GithubUser, google::GoogleUser},
};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Admin {
    #[serde(default)]
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub provider: String,
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
    pub provider: String,
    #[serde(default)]
    pub roles: Vec<Roles>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Roles {
    pub name: String,
    pub permissions: Vec<String>,
}

impl From<GoogleUser> for User {
    fn from(google_user: GoogleUser) -> Self {
        Self {
            name: google_user.display_name,
            email: google_user.email,
            password: "".to_string(),
            roles: vec![],
            provider: Provider::Google.to_string(),
        }
    }
}

impl From<GoogleUser> for Admin {
    fn from(google_user: GoogleUser) -> Self {
        Self {
            name: google_user.display_name,
            email: google_user.email,
            password: "".to_string(),
            roles: vec![],
            provider: Provider::Google.to_string(),
        }
    }
}

impl From<GithubUser> for Admin {
    fn from(github_user: GithubUser) -> Self {
        Self {
            name: github_user.display_name,
            email: github_user.email,
            password: "".to_string(),
            roles: vec![],
            provider: Provider::Github.to_string(),
        }
    }
}

impl From<GithubUser> for User {
    fn from(github_user: GithubUser) -> Self {
        Self {
            name: github_user.display_name,
            email: github_user.email,
            password: "".to_string(),
            roles: vec![],
            provider: Provider::Github.to_string(),
        }
    }
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
            provider: user.provider.to_string(),
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
            provider: admin.provider.to_string(),
        }
    }
}
