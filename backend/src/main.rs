mod models;
mod schema;
mod database;
mod auth;

use database::Database;
use models::NewProduct;
use rocket::{get, launch, routes, http::{Status, CookieJar, Cookie}, post, serde::json::Json, form::Form};
use rocket_sentry::RocketSentry;

use crate::models::{Product, NewUser};

#[get("/get_products")]
async fn get_products(db: Database) -> Json<Vec<Product>> {
    let products = db
        .get_products()
        .await;

    Json(products)
}

#[post("/add_product", data = "<product>")]
fn add_product(_db: Database, product: Json<NewProduct>) -> Status {
    println!("{:?}", product);
    Status::Created
}

#[post("/create_user", data = "<new_user>")]
async fn create_user(db: Database, cookies: &CookieJar<'_>, new_user: Json<NewUser>) -> Status {
    let user = db.create_user(new_user.into_inner()).await;
    println!("user id: {}", user.0);
    Status::Ok
}

#[post("/login", data = "<new_user>")]
fn login(_db: Database, cookies: &CookieJar<'_>, new_user: Json<NewUser>) -> Status {
    Status::Accepted
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Database::fairing())
        .attach(RocketSentry::fairing())
        .mount("/", routes![
            get_products,
            add_product,
            create_user,
            login,
        ])
}
