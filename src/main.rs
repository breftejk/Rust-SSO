mod oauth2;
mod db;
mod errors;
mod auth;

#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;
use rocket::fs::FileServer;
use dotenv::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(Template::fairing())
        .mount("/oauth2", oauth2::routes::get_routes())
        .mount("/auth", auth::routes::get_routes())
        .mount("/static", FileServer::from("static"))
}