use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl Clone for User {
    fn clone(&self) -> User {
        User {
            _id: self._id,
            email: self.email.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
        }
    }
}
