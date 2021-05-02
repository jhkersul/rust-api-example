use mongodb::bson::doc;
use super::Database;
use super::super::domain::User;

impl Database {
  pub async fn save_user(&self, user: User) -> User {
    match &self.users_collection().insert_one(doc! {
        "first_name": user.first_name.clone(),
        "last_name": user.last_name.clone(),
        "email": user.email.clone()
    }, None).await {
        Ok(_) => println!("Saved!"),
        Err(error) => println!("{}", error)
    }

    user
  }
}
