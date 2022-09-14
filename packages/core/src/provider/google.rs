use std::env;

use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleCreateAccCallback {
    code: String,
    state: String,
    scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUser {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub email: String,
    pub picture: String,
}

pub fn create_acc(typee: &'static str) -> String {
    let client = BasicClient::new(
        ClientId::new(env::var("GOOGLE_CLIENT_ID").unwrap()),
        Some(ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").unwrap())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
    )
    .set_redirect_uri(
        RedirectUrl::new(format!(
            "{}/account/{}/create/google/callback",
            env::var("SERVER_ADDR").unwrap(),
            typee
        ))
        .unwrap(),
    );

    let (pkce_challenge, _pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
    let (auth_url, _csrf_token) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("email".to_string()))
        .add_scope(oauth2::Scope::new("profile".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    auth_url.to_string()
}

pub async fn google_callback(callback: GoogleCreateAccCallback) -> GoogleUser {
    let client = BasicClient::new(
        ClientId::new(env::var("GOOGLE_CLIENT_ID").unwrap()),
        Some(ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").unwrap())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
    );

    let token = client
        .exchange_code(oauth2::AuthorizationCode::new(callback.code.clone()))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .unwrap();

    let client = reqwest::Client::new();
    let res = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(token.access_token().secret().clone())
        .send()
        .await
        .unwrap();

    let user: GoogleUser = res.json().await.unwrap();
    return user;
}
