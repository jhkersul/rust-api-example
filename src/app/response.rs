use serde::{Serialize, Deserialize};
use super::domain::User;

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
  pub id: String
}

#[derive(Serialize, Deserialize)]
pub struct GetUserResponse {
  pub id: String,
  pub email: String,
  pub first_name: String,
  pub last_name: String
}

impl GetUserResponse {
  pub fn from_domain(user: &User) -> GetUserResponse {
    GetUserResponse {
      id: user._id.clone().to_string(),
      email: user.email.clone(),
      first_name: user.first_name.clone(),
      last_name: user.last_name.clone(),
    }
  }
}
