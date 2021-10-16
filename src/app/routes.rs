use mongodb::bson::oid::ObjectId;
use rocket::State;
use rocket::serde::json::Json;
use super::{
    db::Database,
    request::CreateUserRequest,
    response::{CreateUserResponse, GetUserResponse}
};
use rocket_dyn_templates::Template;

#[get("/users/<id>")]
pub async fn get_user(
    id: String, db: &State<Database>
) -> Json<GetUserResponse> {
    let object_id = ObjectId::parse_str(&id).unwrap();

    return match db.get_user(object_id).await {
        Some(user) => Json(GetUserResponse::from_domain(&user)),
        None => panic!("User not found")
    }

}

#[get("/users")]
pub async fn get_users(
    db: &State<Database>
) -> Json<Vec<GetUserResponse>> {
    let users = db.get_users(10).await;
    let response = users
        .into_iter()
        .map(|user| GetUserResponse::from_domain(&user))
        .collect();
    Json(response)
}

#[post(
    "/users",
    format = "application/json",
    data = "<create_user_request>"
)]
pub async fn create_user(
    create_user_request: Json<CreateUserRequest>,
    db: &State<Database>
) -> Json<CreateUserResponse> {
    let user = create_user_request.to_domain();
    let user_id = db.save_user(&user).await;

    Json(CreateUserResponse {
        id: user_id.to_string(),
    })
}

#[get("/health", format = "text/html")]
pub async fn health_check() -> String {
    "Health OK".to_string()
}

#[get("/")]
pub async fn root() -> Template {
    Template::render("index", ())
}


#[cfg(test)]
mod test {
    use mongodb::bson::oid::ObjectId;
    use crate::app::{
        db::Database,
        domain::User,
        response::{CreateUserResponse, GetUserResponse}
    };
    use super::super::super::rocket;
    use rocket::{http::ContentType, local::asynchronous::Client};
    use rocket::http::Status;

    #[rocket::async_test]
    async fn should_get_user() {
        let db = Database::init().await;
        let user = User {
            _id: ObjectId::new(),
            email: "test@test.com".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string()
        };
        db.save_user(&user).await;
        let client = Client::tracked(rocket().await).await.unwrap();
        let path = format!("/users/{}", user._id.to_string());

        let response = client.get(path).dispatch().await;

        assert_eq!(response.status(), Status::Ok);
        let response_json = response.into_string().await.unwrap();
        let get_user_response: GetUserResponse = serde_json::
            from_str(response_json.as_str())
            .unwrap();
        assert_eq!(get_user_response.id, user._id.to_string());
        assert_eq!(get_user_response.email, user.email);
    }

    #[rocket::async_test]
    async fn should_get_users() {
        let db = Database::init().await;
        let user1 = User {
            _id: ObjectId::new(),
            email: "test@test.com".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string()
        };
        let user2 = User {
            _id: ObjectId::new(),
            email: "test@test.com".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string()
        };
        db.save_user(&user1).await;
        db.save_user(&user2).await;
        let client = Client::tracked(rocket().await).await.unwrap();
        let path = "/users";

        let response = client.get(path).dispatch().await;

        assert_eq!(response.status(), Status::Ok);
        let response_json = response.into_string().await.unwrap();
        let response: Vec<GetUserResponse> = serde_json::
            from_str(response_json.as_str())
            .unwrap();
        assert!(response[0].id.len() > 0);
        assert!(response[1].id.len() > 0);
    }

    #[rocket::async_test]
    async fn should_create_user() {
        let client = Client::tracked(rocket().await).await.unwrap();
        let json_body = r#"
            {
                "email": "test@test.com",
                "first_name": "John",
                "last_name": "Doe"
            }"#;

        let response = client
            .post("/users")
            .body(json_body)
            .header(ContentType::JSON)
            .dispatch()
            .await;
        let response_json = response.into_string().await.unwrap();
        let create_user_response: CreateUserResponse = serde_json::
            from_str(response_json.as_str())
            .unwrap();

        assert_eq!(create_user_response.id.len() > 0, true);
    }

    #[rocket::async_test]
    async fn should_health_check() {
        let client = Client::tracked(rocket().await).await.unwrap();

        let response = client
            .get("/health")
            .header(ContentType::Text)
            .dispatch().await;
        let response_string = response.into_string().await.unwrap();

        assert_eq!(response_string, "Health OK")
    }

    #[rocket::async_test]
    async fn should_check_root_route() {
        let client = Client::tracked(rocket().await).await.unwrap();

        let response = client
            .get("/")
            .header(ContentType::Text)
            .dispatch().await;
        let response_string = response.into_string().await.unwrap();

        assert!(
            response_string
            .contains("<h1>Welcome to Rust API Example</h1>")
        )
    }
}
