#[macro_use]
extern crate rocket;

mod app;

use crate::app::db::user::UsersRepository;
use crate::app::db::Database;
use app::routes as app_routes;
use rocket::Build;
use rocket::Rocket;
use rocket_dyn_templates::Template;

#[rocket::main]
async fn main() {
    rocket().await.launch().await.unwrap()
}

async fn rocket() -> Rocket<Build> {
    let db = Database::new().await;
    let users_repo = UsersRepository::new(db);

    rocket::build()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                app_routes::get_user,
                app_routes::get_users,
                app_routes::create_user,
                app_routes::health_check,
                app_routes::root
            ],
        )
        .manage(users_repo)
}
