mod models;
mod schema;
mod database;
mod routes;

use database::Database;
use rocket::{launch, routes, fs::{FileServer, relative}};
use rocket_sentry::RocketSentry;
use routes::{index, products, users};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Database::fairing())
        .attach(RocketSentry::fairing())
        .mount("/", routes![
            index::register,
            index::login,
            index::logout,

            products::get_products,
            products::remove_product,
            products::add_product,

            users::get_users,
        ])
        .mount("/public", FileServer::from(relative!("static")))
}
