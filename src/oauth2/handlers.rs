use std::str::FromStr;

use rocket::get;
use rocket::serde::json::Json;

use crate::errors::{ErrorResponse, http_error};
use crate::oauth2::models::client::{Client, ClientScope, get_client, get_clients};

#[get("/clients")]
pub async fn index() -> Json<Vec<Client>> {
    Json(get_clients().await)
}

#[derive(FromForm)]
pub struct AuthenticationRequest {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    state: String,
}

#[get("/?<scope>&<auth_request..>")]
pub async fn authentication_request(
    auth_request: Option<AuthenticationRequest>,
    scope: Option<String>,
) -> Result<Json<Vec<ClientScope>>, ErrorResponse> {
    let auth_request_data = auth_request.unwrap();

    let client = match get_client(&auth_request_data.client_id).await {
        Some(client) => client,
        None => return Err(http_error(404, "Client not found")),
    };

    if !&client.secret.eq(&auth_request_data.client_secret) {
        return Err(http_error(401, "Client secret is incorrect"));
    }

    if !&client.redirect_uris.iter().any(|url| url.eq(&auth_request_data.redirect_uri)) {
        return Err(http_error(400, "Redirect URI is not allowed"));
    }

    let scopes: Vec<ClientScope> = scope
        .unwrap_or_default()
        .split(" ")
        .filter_map(|s| {
            let scope = ClientScope::from_str(s.trim()).ok();
            scope
        })
        .collect();

    Ok(Json(scopes))
}
