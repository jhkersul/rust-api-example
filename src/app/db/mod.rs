use mongodb::{
  Client,
  options::ClientOptions,
  Database as MongoDatabase,
  Collection
};

const DATABASE_URL: &str = "mongodb://root:root@localhost:27017";
const APP_NAME: &str = "rust-api-example";
const DATABASE_NAME: &str = "rust-api-example";

mod user;

pub struct Database {
  client: Client
}

impl Database {
  pub async fn init() -> Self {
    let mut client_options = ClientOptions::parse(DATABASE_URL).await.unwrap();
    client_options.app_name = Some(APP_NAME.to_string());

    Self {
      client: Client::with_options(client_options).unwrap()
    }
  }

  pub fn users_collection(&self) -> Collection {
    self.database().collection("users")
  }

  fn database(&self) -> MongoDatabase {
    self.client.database(DATABASE_NAME)
  }
}
