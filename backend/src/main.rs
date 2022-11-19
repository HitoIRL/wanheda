mod models;
mod schema;
mod database;

use database::Database;
use models::NewProduct;
use rocket::{get, launch, routes, http::Status, post, serde::json::Json};
use rocket_sentry::RocketSentry;

use crate::models::Product;

#[get("/panic")]
fn pp() {
    panic!("giga test sentry");
}

#[get("/get_products", format = "json")]
async fn get_products(db: Database) -> Json<Vec<Product>> {
    let products = db
        .get_products()
        .await
        .unwrap(); // TODO: handle errors

    Json(products)
}

#[post("/add_product", format = "json", data = "<product>")]
fn add_product(_db: Database, product: Json<NewProduct>) -> Status {
    println!("{:?}", product);
    Status::Created
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Database::fairing())
        .attach(RocketSentry::fairing())
        .mount("/", routes![
            get_products,
            add_product,
            pp
        ])
}
