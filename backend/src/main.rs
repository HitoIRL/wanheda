mod models;
mod schema;
mod database;
mod routes;

use database::Database;
use rocket::{launch, routes, fs::{FileServer, relative}, fairing::{Fairing, Info, Kind}, Request, Response, http::Header};
use rocket_sentry::RocketSentry;
use routes::{index, products, users};

struct CORS;

#[rocket::async_trait]
impl Fairing for CORS { // https://stackoverflow.com/questions/62412361/how-to-set-up-cors-or-options-for-rocket-rs
    fn info(&self) -> Info {
        Info {
            name: "CORS headers",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:8080")); // TODO: use figment or env variables to fetch it
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, DELETE"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Database::fairing())
        .attach(RocketSentry::fairing())
        .attach(CORS)
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
