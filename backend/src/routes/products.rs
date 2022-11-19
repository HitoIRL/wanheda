use rocket::{get, delete, post, serde::json::Json, http::Status};

use crate::{database::Database, models::{Product, User, NewProduct}};

#[get("/products")]
pub async fn get_products(db: Database) -> Json<Vec<Product>> {
    let products = db
        .get_products()
        .await;

    Json(products)
}

#[delete("/products/<id>")]
pub async fn remove_product(db: Database, user: User, id: i32) -> Status {
    if user.is_admin {
        db.remove_product(id).await;
        Status::Ok
    } else {
        Status::Forbidden
    }
}

#[post("/products", data = "<product>")]
pub async fn add_product(db: Database, user: User, product: Json<NewProduct>) -> Result<Json<Product>, Status> {
    if user.is_admin {
        let product = db.add_product(product.into_inner()).await;
        Ok(Json(product))
    } else {
        Err(Status::Forbidden)
    }
}
