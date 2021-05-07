extern crate dotenv;
extern crate mongodb;

use dotenv::dotenv;
use mongodb::{Client, Database, error::{Error}};

pub async fn connect() -> Result<Database, Error> {
    dotenv().ok();
    let mongo_url = dotenv::var("MONGO_URL").unwrap();
    let client = Client::with_uri_str(&mongo_url).await?;
    return Ok(client.database("vtt_bzh"));
}
