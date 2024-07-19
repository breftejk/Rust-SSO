use anyhow;
use oauth2::{AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields, PkceCodeChallenge, RedirectUrl, RequestTokenError, Scope, TokenResponse, TokenUrl};
use oauth2::basic::{BasicClient, BasicRequestTokenError};
use oauth2::reqwest::async_http_client;
use oauth2::StandardRevocableToken::AccessToken;
use rocket::http::Status;
use rocket::response::Redirect;
use url::Url;
use dotenv::dotenv;

use crate::auth::discord;
use crate::errors::{ErrorResponse, http_error, HttpError};

#[get("/discord")]
pub async fn discord_auth() -> Redirect {
    let url = discord::get_url();
    Redirect::to(url.to_string())
}

#[get("/discord/callback?<code>")]
pub async fn discord_callback(code: Option<String>) -> Result<String, ErrorResponse> {
    Ok(discord::get_token(code.unwrap()).await?.access_token().secret().to_string())
}