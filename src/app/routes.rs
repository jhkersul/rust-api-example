use rocket::State;
use rocket_contrib::json::Json;
use super::{db::Database, request::CreateUserRequest};

#[get("/users")]
pub fn get_users() -> &'static str {
    "World!"
}

#[post("/users", format = "application/json", data = "<create_user_request>")]
pub async fn create_user(create_user_request: Json<CreateUserRequest>, db: State<'_, Database>) -> String {
    let user = create_user_request.to_domain();
    let saved_user = db.save_user(user).await;

    format!(
        "{} {} {}",
        saved_user.email,
        saved_user.first_name,
        saved_user.last_name
    )
}

#[cfg(test)]
mod test {
    use super::super::super::rocket;
    use rocket::{http::ContentType, local::asynchronous::Client};
    use rocket::http::Status;

    #[rocket::async_test]
    async fn should_get_users() {
        let client = Client::tracked(rocket().await).await.unwrap();

        let response = client.get("/users").dispatch().await;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "World!".to_string());
    }

    #[rocket::async_test]
    async fn should_create_hello() {
        let client = Client::tracked(rocket().await).await.unwrap();
        let json_body = r#"
            {
                "email": "test@test.com",
                "first_name": "John",
                "last_name": "Doe"
            }"#;
        
        let response = client.post("/users").body(json_body).header(ContentType::JSON).dispatch().await;

        assert_eq!(response.into_string().await.unwrap(), "test@test.com John Doe".to_string());
    }
}
