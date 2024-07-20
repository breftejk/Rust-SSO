use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ExternalIdentityProvider {
    Discord,
    Google,
    Apple,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalIdentity {
    pub user_id: String,
    pub provider: ExternalIdentityProvider,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionState {
    AwaitingRegistrationFromExternalProvider {
        external_identity: ExternalIdentity,
    },

    LoggedInAwaiting2FACode {
        user_id: String,
    },

    UserSessionActive {
        user_id: String,
    },
}