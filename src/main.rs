#[macro_use] extern crate rocket;

use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

mod oauth2;
mod db;
mod errors;
mod auth;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(AdHoc::config::<rocket::Config>())
        .attach(Template::fairing())
        .mount("/oauth2", oauth2::routes::get_routes())
        .mount("/auth", auth::routes::get_routes())
        .mount("/static", FileServer::from("static"))
}