use super::request::CreateUserRequest;
use super::domain::User;
use super::repo::save_user;
use rocket_contrib::json::Json;

#[get("/users")]
pub fn get_users() -> &'static str {
    "World!"
}

#[post("/users", format = "application/json", data = "<create_user_request>")]
pub fn create_user(create_user_request: Json<CreateUserRequest>) -> String {
    let user = create_user_request.to_domain();
    let saved_user: User = save_user(user);

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
    use rocket::local::Client;
    use rocket::http::ContentType;
    use rocket::http::Status;

    #[test]
    fn should_get_users() {
        let client = Client::new(rocket()).expect("valid rocket instance");

        let mut response = client.get("/users").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("World!".to_string()));
    }

    #[test]
    fn should_create_hello() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let json_body = r#"
            {
                "email": "test@test.com",
                "first_name": "John",
                "last_name": "Doe"
            }"#;
        
        let mut response = client.post("/users").body(json_body).header(ContentType::JSON).dispatch();

        assert_eq!(response.body_string(), Some("test@test.com John Doe".to_string()));
    }
}
