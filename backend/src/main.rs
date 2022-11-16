/*
#[macro_use] extern crate rocket;

#[get("/collection")]
fn collection() -> String {
    "{collection}".to_owned()
}

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();
    rocket::build().mount("/", routes![collection])
}
*/

mod models;
mod schema;

use std::env;

use diesel::prelude::*;
use dotenvy::dotenv;

use crate::models::Product;

fn establish_connection() -> PgConnection {
    let url = env::var("DATABASE_URL").expect("Couldnt find DATABASE_URL in your .env file");
    PgConnection::establish(&url).expect("Failed to connect to the database")
}

fn main() {
    dotenv().expect("Failed to load .env file");

    let connection = &mut establish_connection();

    use self::schema::products::dsl::*;
    let results = products
        .limit(3)
        .load::<Product>(connection)
        .expect("Failed loading products");

    println!("Found {} products", results.len());
    for product in results {
        println!("{}", product.title);
    }
}
