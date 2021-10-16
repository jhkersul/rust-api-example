use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    results::InsertOneResult
};
use rocket::futures::stream::TryStreamExt;
use super::Database;
use super::super::domain::User;


fn get_id(result: &InsertOneResult) -> ObjectId {
    match result.inserted_id.as_object_id() {
        Some(object_id) => object_id,
        None => panic!("No id was returned")
    }
}

impl Database {
    pub async fn save_user(&self, user: &User) -> ObjectId {
        let id = match &self
            .users_collection()
            .insert_one(user, None)
            .await {
                Ok(result) => get_id(result),
                Err(error) => panic!("{}", error)
            };

        id
    }

    pub async fn get_user(&self, id: ObjectId) -> Option<User> {
        let filter = doc! { "_id": id };

        match &self.users_collection().find_one(filter, None).await {
            Ok(user) => self.remove_ref_user(user),
            Err(error) => panic!("{}", error)
        }
    }

    pub async fn get_users(&self, limit: i64) -> Vec<User> {
        let find_options = FindOptions::builder().limit(limit).build();
        let find = self.users_collection()
            .find(doc! {}, find_options)
            .await;

        match find {
            Ok(cursor) => cursor
                .try_collect()
                .await
                .unwrap_or_else(|_| vec![]),
            Err(error) => panic!("{}", error)
        }
    }
}

#[cfg(test)]
mod test {
    use mongodb::bson::oid::ObjectId;
    use super::Database;
    use super::User;

    #[rocket::async_test]
    async fn should_create_user() {
        let db = Database::init().await;
        let user = User {
            _id: ObjectId::new(),
            email: "test@test.com".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string()
        };

        let saved_id = db.save_user(&user).await;

        match db.get_user(saved_id).await {
            Some(saved_user) => {
                assert_eq!(user._id, saved_user._id);
                assert_eq!(user.email, saved_user.email);
                assert_eq!(user.first_name, saved_user.first_name);
                assert_eq!(user.last_name, saved_user.last_name);
            }
            None => panic!("Failed get user")
        }
    }
}

