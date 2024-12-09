
use std::env;

use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct MongoClient {
    pub client: Arc<Client>,
}

pub async fn new() -> Result<MongoClient,  Box<dyn std::error::Error>> {
    let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    println!("Connecting to {}", mongo_uri);

    let options =
        ClientOptions::parse_with_resolver_config(&mongo_uri, ResolverConfig::cloudflare()).await?;

    let client = Client::with_options(options)?;

    let mongo_client = MongoClient {
        client: Arc::new(client)
    };

    Ok(mongo_client)
}