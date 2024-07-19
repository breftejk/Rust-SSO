use rocket::Route;

use crate::oauth2::handlers::*;

pub fn get_routes() -> Vec<Route> {
    routes![index, authentication_request]
}