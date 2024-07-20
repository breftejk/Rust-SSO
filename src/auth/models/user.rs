use mongodb::bson::{oid::ObjectId, doc};
use mongodb::Collection;
use rocket::serde::{Deserialize, Serialize};

use crate::auth::models::session::ExternalIdentity;
use crate::db::get_mongo_client;

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorSettings {
    pub enabled: bool,
    pub devices: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExternalIdentityData {
    user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalIdentities {
    discord: ExternalIdentityData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub email: String,
    pub two_factor_settings: TwoFactorSettings,

    pub external_identities: ExternalIdentities,
}

impl User {
    pub async fn find_by_external_identity(external_identity: &ExternalIdentity) -> Option<User> {
        let mongo_client = get_mongo_client().await;
        let coll: Collection<User> = mongo_client.database("SSO").collection("users");

        let filter = doc! { "external_identities.discord.user_id": &external_identity.user_id };
        let user_result = coll.find_one(filter).await;

        user_result.ok().flatten()
    }

    pub async fn create_with_external_identity(external_identity: ExternalIdentity, username: String, email: String, password: String) -> Option<User> {
        let mongo_client = get_mongo_client().await;
        let coll: Collection<User> = mongo_client.database("SSO").collection("users");

        let user = User {
            _id: ObjectId::new(),
            username,
            email,
            two_factor_settings: TwoFactorSettings {
                enabled: false,
                devices: Vec::new(),
            },
            external_identities: ExternalIdentities {
                discord: ExternalIdentityData {
                    user_id: external_identity.user_id.clone(),
                },
            },
        };

        coll.insert_one(&user).await.expect("TODO: panic message");

        Some(user)
    }
}