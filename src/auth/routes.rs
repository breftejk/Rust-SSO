use rocket::Route;

use crate::auth::handlers::*;

pub fn get_routes() -> Vec<Route> {
    routes![discord_auth, discord_callback]
}