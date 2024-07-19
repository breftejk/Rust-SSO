use std::env;
use oauth2::{AuthorizationCode, AuthUrl, Client, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, StandardRevocableToken, TokenUrl};
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse, BasicTokenType};
use oauth2::reqwest::async_http_client;
use url::Url;

use crate::errors::{ErrorResponse, http_error};

fn get_client() -> Client<BasicErrorResponse, BasicTokenResponse, BasicTokenType, BasicTokenIntrospectionResponse, StandardRevocableToken, BasicRevocationErrorResponse> {
    BasicClient::new(
        ClientId::new(env::var("AUTH_DISCORD_CLIENT_ID").expect("AUTH_DISCORD_CLIENT_ID must be set")),
        Some(ClientSecret::new(env::var("AUTH_DISCORD_CLIENT_SECRET").expect("AUTH_DISCORD_CLIENT_SECRET must be set"))),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).unwrap()),
    )
        .set_redirect_uri(RedirectUrl::new(env::var("AUTH_DISCORD_REDIRECT_URI").expect("AUTH_DISCORD_REDIRECT_URI must be set")).unwrap())
}

pub fn get_url() -> Url {
    let client = get_client();

    let (auth_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    auth_url
}

pub async fn get_token(code: String) -> Result<BasicTokenResponse, ErrorResponse> {
    let client = get_client();

    client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await
        .map_err(|e| http_error(500, &format!("Failed to exchange code: {}", e)))
}