use super::{super::domain::User, Database};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    results::InsertOneResult,
    Collection,
};
use rocket::futures::stream::TryStreamExt;

fn get_id(result: &InsertOneResult) -> ObjectId {
    match result.inserted_id.as_object_id() {
        Some(object_id) => object_id,
        None => panic!("No id was returned"),
    }
}

pub struct UsersRepository {
    database: Database,
}

impl UsersRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn save_user(&self, user: &User) -> ObjectId {
        let id = match &self.users_collection().await.insert_one(user, None).await {
            Ok(result) => get_id(result),
            Err(error) => panic!("{}", error),
        };

        id
    }

    pub async fn get_user(&self, id: ObjectId) -> Option<User> {
        let filter = doc! { "_id": id };

        match &self.users_collection().await.find_one(filter, None).await {
            Ok(user) => user.clone(),
            Err(error) => panic!("{}", error),
        }
    }

    pub async fn get_users(&self, limit: i64) -> Vec<User> {
        let find_options = FindOptions::builder().limit(limit).build();
        let find = self
            .users_collection()
            .await
            .find(doc! {}, find_options)
            .await;

        match find {
            Ok(cursor) => cursor.try_collect().await.unwrap_or_else(|_| vec![]),
            Err(error) => panic!("{}", error),
        }
    }

    pub async fn users_collection(&self) -> Collection<User> {
        self.database.collection("users")
    }
}

#[cfg(test)]
mod test {
    use super::UsersRepository;
    use crate::app::{db::Database, domain::User};
    use mongodb::bson::oid::ObjectId;

    #[rocket::async_test]
    async fn should_create_user() {
        let user = User {
            _id: ObjectId::new(),
            email: "test@test.com".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        let db = Database::new().await;
        let users_repo = UsersRepository::new(db);

        let saved_id = users_repo.save_user(&user).await;

        match users_repo.get_user(saved_id).await {
            Some(saved_user) => {
                assert_eq!(user._id, saved_user._id);
                assert_eq!(user.email, saved_user.email);
                assert_eq!(user.first_name, saved_user.first_name);
                assert_eq!(user.last_name, saved_user.last_name);
            }
            None => panic!("Failed get user"),
        }
    }
}
