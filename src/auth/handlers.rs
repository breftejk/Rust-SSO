use oauth2::TokenResponse;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};
use url::quirks::password;

use crate::auth::discord;
use crate::auth::models::session::{ExternalIdentity, ExternalIdentityProvider, SessionState};
use crate::auth::models::user::User;
use crate::errors::{ErrorResponse, http_error};

#[get("/discord")]
pub async fn discord_auth() -> Redirect {
    let url = discord::get_url();
    Redirect::to(url.to_string())
}

#[get("/discord/callback?<code>")]
pub async fn discord_callback(
    code: Option<String>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, ErrorResponse> {
    let token = discord::get_token(code.unwrap()).await?.access_token().secret().to_string();

    if let Ok(discord_user) = discord::get_user(token).await {
        let provider_identity = ExternalIdentity {
            user_id: discord_user.id,
            provider: ExternalIdentityProvider::Discord,
        };
        if let Some(user) = User::find_by_external_identity(&provider_identity).await {
            if user.two_factor_settings.enabled {
                let session_state = SessionState::LoggedInAwaiting2FACode {
                    user_id: user._id.to_string(),
                };
                let session_state_string = serde_json::to_string(&session_state).unwrap();
                cookies.add_private(Cookie::new("session_state", session_state_string));

                Ok(Redirect::to("/auth/process"))
            } else {
                let session_state = SessionState::UserSessionActive {
                    user_id: user._id.to_string(),
                };

                let session_state_string = serde_json::to_string(&session_state).unwrap();
                cookies.add_private(Cookie::new("session_state", session_state_string));

                Ok(Redirect::to("/auth/process"))
            }
        } else {
            let session_state = SessionState::AwaitingRegistrationFromExternalProvider {
                external_identity: provider_identity,
            };

            let session_state_string = serde_json::to_string(&session_state).unwrap();
            let cookie = Cookie::build(("session_state", session_state_string))
                .http_only(false)
                .same_site(SameSite::Lax)
                .secure(false);
            cookies.add_private(cookie);

            Ok(Redirect::to(uri!("/auth", process_session_state)))
        }
    } else {
        Err(http_error(500, "Couldn't get user data from Discord"))
    }
}

#[get("/process")]
pub async fn process_session_state(
    cookies: &CookieJar<'_>
) -> Result<Template, ErrorResponse> {
    if let Some(cookie) = cookies.get_private("session_state") {
        let session_state: SessionState = serde_json::from_str(&cookie.value()).unwrap();

        if let SessionState::UserSessionActive { user_id } = &session_state {
            return Err(http_error(500, "Couldn't. Session is active."))
        }

        Ok(Template::render("additional", context! {
            session_state: session_state,
            error: context! {
                bool: false,
                message: ""
            }
        }))
    } else {
        Ok(Template::render("additional", context! {
            error: context! {
                bool: true,
                message: "Session state cookie not found".to_string(),
            },
        }))
    }
}

#[derive(FromForm)]
struct ExternalAdditional<'r> {
    #[field(name = uncased("username"))]
    username: &'r str,

    #[field(name = uncased("email"))]
    #[field(name = uncased("e-mail"))]
    email: &'r str,

    #[field(name = uncased("password"))]
    #[field(name = uncased("pass"))]
    password: &'r str,
}

#[post("/additional", data = "<external_data>")]
pub async fn submit_external_additional(
    cookies: &CookieJar<'_>,
    external_data: Option<Form<ExternalAdditional<'_>>>
) -> Result<String, ErrorResponse> {
    let form_data = external_data.unwrap().into_inner();

    if let Some(cookie) = cookies.get_private("session_state") {
        let session_state: SessionState = serde_json::from_str(&cookie.value()).unwrap();

        match session_state {
            SessionState::AwaitingRegistrationFromExternalProvider { external_identity } => {
                let user = User::create_with_external_identity(
                    external_identity,
                    form_data.username.to_string(),
                    form_data.email.to_string(),
                    form_data.password.to_string()
                ).await.unwrap();

                cookies.remove_private("session_state");

                let new_state = SessionState::UserSessionActive {
                    user_id: String::from(&user._id.to_string()),
                };

                let cookie = Cookie::build(("session_state", serde_json::to_string(&new_state).unwrap()))
                    .http_only(false)
                    .same_site(SameSite::Lax)
                    .secure(false);
                cookies.add_private(cookie);

                Ok(user._id.to_string())
            }
            _ => return Err(http_error(401, "Session does not require that")),
        }
    } else {
        Ok(format!("Failed to set cookie {}", 4))
    }
}