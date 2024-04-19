use mongodb::{Client, options::ClientOptions, error::Result};

pub async fn connect(
    conn_str: &str
) -> Result<Client> {
    let client_options = ClientOptions::parse(conn_str).await?;
    print!("Connecting to MongoDB...");
    Client::with_options(client_options)
}
