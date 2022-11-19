use crate::models::{Product, NewProduct, User, NewUser};

use rocket_sync_db_pools::{diesel, database};
use diesel::prelude::*;

#[database("wanheda")]
pub struct Database(diesel::PgConnection);

impl Database {
    // products
    pub async fn get_products(&self) -> Vec<Product> {
        use crate::schema::products::dsl::*;
    
        self.run(move |conn| {
            products
                .load(conn)
                .unwrap()
        }).await
    }

    pub async fn add_product(&self, product: NewProduct) -> Product {
        use crate::schema::products;
    
        self.run(move |conn| {
            diesel::insert_into(products::table)
                .values(&product)
                .get_result(conn)
                .unwrap()
        }).await
    }

    pub async fn remove_product(&self, product_id: i32) {
        self.run(move |conn| {
            use crate::schema::products::dsl::*;

            diesel::delete(products.filter(id.eq(&product_id))).execute(conn).unwrap();
        }).await
    }

    // users
    pub async fn get_users(&self) -> Vec<User> {
        use crate::schema::users::dsl::*;

        self.run(move |conn| {
            users
                .load(conn)
                .unwrap()
        }).await
    }

    pub async fn get_user(&self, user_info: NewUser) -> Option<User> {
        use crate::schema::users::dsl::*;

        self.run(move |conn| {
            users
                .filter(username.eq(user_info.username))
                .first(conn)
                .optional()
                .unwrap()
        }).await
    }

    pub async fn create_user(&self, new_user: NewUser) -> User {
        use crate::schema::users;

        self.run(move |conn| {
            diesel::insert_into(users::table)
                .values(&new_user)
                .get_result(conn)
                .unwrap()
        }).await
    }
}
