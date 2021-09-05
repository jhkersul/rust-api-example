#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod app;

use app::routes as app_routes;
use app::db::Database;
use rocket::Rocket;
use rocket::Build;

#[rocket::main] 
async fn main() {
    match rocket().await.launch().await {
        Ok(_) => {()}
        Err(_) => {()}
    };
}

async fn rocket() -> Rocket<Build> {
    let db = Database::init().await;

    return rocket::build()
            .mount(
                "/",
                routes![
                    app_routes::get_user,
                    app_routes::create_user,
                    app_routes::health_check
                ]
            )
            .manage(db)
}
