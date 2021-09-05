use serde::Deserialize;
use super::domain::User;

#[derive(Deserialize)]
pub struct CreateUserRequest {
  pub email: String,
  pub first_name: String,
  pub last_name: String
}

impl CreateUserRequest {
  pub fn to_domain(&self) -> User {
    User {
      _id: None,
      email: self.email.clone(),
      first_name: self.first_name.clone(),
      last_name: self.last_name.clone()
    }    
  }
}
