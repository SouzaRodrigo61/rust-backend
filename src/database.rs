use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mongodb::Database;
use tracing::debug;
use wither::mongodb::{self, options::ClientOptions};

use crate::settings::SETTINGS;

lazy_static! {
  pub static ref CONNECTION: AsyncOnce<Database> = AsyncOnce::new(async {
    let db_uri = SETTINGS.database.uri.as_str();
    let db_name = SETTINGS.database.name.as_str();

    debug!("Connecting to MongoDB...");

    let client_options = ClientOptions::parse(db_uri)
      .await
      .expect("Failed to parse MongoDB connection string");

    debug!("Connected to MongoDB...");

    mongodb::Client::with_options(client_options)
      .expect("Failed to initialize MongoDB connection")
      .database(db_name)
  });
}
