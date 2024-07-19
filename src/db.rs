use std::env;
use mongodb::Client;
use once_cell::sync::Lazy;
use tokio::sync::OnceCell;

static MONGO_CLIENT: Lazy<OnceCell<Client>> = Lazy::new(OnceCell::new);

async fn init_mongo_client() -> Client {
    let uri = env::var("MONGO_URL").expect("MONGO_URL must be set");
    Client::with_uri_str(uri).await.expect("Failed to initialize MongoDB client")
}

pub async fn get_mongo_client() -> &'static Client {
    MONGO_CLIENT.get_or_init(init_mongo_client).await
}