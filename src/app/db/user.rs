use mongodb::{bson::{Document, doc, oid::ObjectId}, results::InsertOneResult};
use mongodb::bson;
use super::Database;
use super::super::domain::User;


fn get_id(result: &InsertOneResult) -> ObjectId {
  match result.inserted_id.as_object_id() {
      Some(object_id) => object_id.clone(),
      None => panic!("No id was returned")
  }
}

fn serialize_user(document: &Option<Document>) -> Option<User> {
  match document {
      Some(user_document) => bson::from_document(user_document.clone()).unwrap(),
      None => None
  }
}

fn deserialize_user(user: &User) -> Document {
  let mut result = bson::to_document(user).unwrap();
  result.remove("_id");

  result
}

impl Database {
  pub async fn save_user(&self, user: &User) -> ObjectId {
    let id = match &self
      .users_collection()
      .insert_one(deserialize_user(user), None)
      .await {
          Ok(result) => get_id(result),
          Err(error) => panic!("{}", error)
      };

    id
  }

  pub async fn get_user(&self, id: ObjectId) -> Option<User> {
    let filter = doc! { "_id": id };

    match &self.users_collection().find_one(filter, None).await {
        Ok(document) => serialize_user(document),
        Err(error) => panic!("{}", error)
    }
  }
}

#[cfg(test)]
mod test {
  use super::Database;
  use super::User;

  #[rocket::async_test]
  async fn should_create_user() {
    let db = Database::init().await;
    let user = User {
      _id: None,
      email: "test@test.com".to_string(),
      first_name: "John".to_string(),
      last_name: "Doe".to_string()
    };

    let saved_id = db.save_user(&user).await;
    
    match db.get_user(saved_id).await {
        Some(saved_user) => {
          assert_eq!(saved_user._id.is_some(), true);
          assert_eq!(user.email, saved_user.email);
          assert_eq!(user.first_name, saved_user.first_name);
          assert_eq!(user.last_name, saved_user.last_name);
        }
        None => panic!("Failed get user")
    }
  }
}
