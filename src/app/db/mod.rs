use mongodb::{options::ClientOptions, Client, Database as MongoDatabase};

const DATABASE_URL: &str = "mongodb://root:root@localhost:27017";
const APP_NAME: &str = "rust-api-example";
const DATABASE_NAME: &str = "rust-api-example";

mod user;

pub struct Database {
    client: Client,
}

impl Database {
    pub async fn new() -> Database {
        let mut client_options = ClientOptions::parse(DATABASE_URL).await.unwrap();
        client_options.app_name = Some(APP_NAME.to_string());

        Self {
            client: Client::with_options(client_options).unwrap(),
        }
    }

    fn database(&self) -> MongoDatabase {
        self.client.database(DATABASE_NAME)
    }
}
