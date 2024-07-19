use std::str::FromStr;

use mongodb::{bson::doc, Collection};
use rocket::futures::TryStreamExt;
use serde::{Deserialize, Serialize};

use crate::db::get_mongo_client;

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientScope {
    UserRead,
    UserWrite,
}

impl FromStr for ClientScope {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UserRead" => Ok(ClientScope::UserRead),
            "UserWrite" => Ok(ClientScope::UserWrite),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub name: String,
    pub id: String,
    pub secret: String,
    pub redirect_uris: Vec<String>,
    pub allowed_scopes: Vec<ClientScope>,
}

impl Client {
    pub async fn new(name: String, id: String, secret: String, redirect_uris: Vec<String>, allowed_scopes: Vec<ClientScope>) -> Self {
        let client = Self {
            name,
            id,
            secret,
            redirect_uris,
            allowed_scopes,
        };

        let mongo_client = get_mongo_client().await;
        let coll: Collection<Client> = mongo_client.database("SSO").collection("clients");

        coll.insert_one(&client).await.expect("TODO: panic message");

        client
    }
}

pub async fn get_clients() -> Vec<Client> {
    let mongo_client = get_mongo_client().await;
    let coll: Collection<Client> = mongo_client.database("SSO").collection("clients");

    let mut clients = Vec::new();

    let mut cursor = coll.find(
        doc! { }
    ).await.unwrap();

    while let Ok(Some(doc)) = cursor.try_next().await {
        clients.push(doc)
    }

    clients
}

pub async fn get_client(client_id: &str) -> Option<Client> {
    let mongo_client = get_mongo_client().await;
    let coll: Collection<Client> = mongo_client.database("SSO").collection("clients");

    let filter = doc! { "id": client_id };
    let client_result = coll.find_one(filter).await;

    client_result.ok().flatten()
}