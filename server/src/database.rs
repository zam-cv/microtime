use anyhow::Result;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};

pub const CLIENT_URI: &str = "mongodb://localhost:27017";

pub async fn init() -> Result<Client> {
    let options =
        ClientOptions::parse_with_resolver_config(&CLIENT_URI, ResolverConfig::cloudflare())
            .await?;

    let client = Client::with_options(options)?;

    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    Ok(client)
}