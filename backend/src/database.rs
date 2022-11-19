use crate::models::{Product, NewProduct};

use rocket_sync_db_pools::{diesel, database};
use diesel::{RunQueryDsl, result::Error};

#[database("wanheda")]
pub struct Database(diesel::PgConnection);

impl Database {
    pub async fn get_products(&self) -> Result<Vec<Product>, Error> {
        use crate::schema::products::dsl::*;
    
        self.run(move |conn| {
            products.load(conn)
        }).await
    }

    pub async fn add_product(&self, title: String, price: f32) -> Product {
        use crate::schema::products;
    
        let new_product = NewProduct {
            title,
            price,
        };
    
        self.run(move |conn| {
            diesel::insert_into(products::table)
                .values(&new_product)
                .get_result(conn)
                .expect("Error adding product")
        }).await
    }
}
