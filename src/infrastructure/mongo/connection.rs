use anyhow::Result;
use mongodb::{Client, Database};
use tracing::info;

pub type MongoDb = Database;

pub async fn init_mongo(uri: &str, db_name: &str) -> Result<Database> {
    let client = Client::with_uri_str(uri).await?;
    info!("DB mongo client ready");
    Ok(client.database(db_name))
}
