#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod app;

use app::routes as app_routes;

fn main() {
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/",
        routes![app_routes::get_users, app_routes::create_user]
    )
}
