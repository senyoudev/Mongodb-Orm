use mongodb::{Client, options::ClientOptions, error::Result};

pub async fn connect() -> Result<Client> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    print!("Connecting to MongoDB...");
    Client::with_options(client_options)
}
