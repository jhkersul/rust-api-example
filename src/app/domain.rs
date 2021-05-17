use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
  pub _id: Option<ObjectId>,
  pub email: String,
  pub first_name: String,
  pub last_name: String
}
