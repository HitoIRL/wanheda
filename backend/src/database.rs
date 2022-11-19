use crate::{models::{Product, NewProduct, UserData, NewUser}, auth::User};

use rocket_sync_db_pools::{diesel, database};
use diesel::RunQueryDsl;

#[database("wanheda")]
pub struct Database(diesel::PgConnection);

impl Database {
    // products
    pub async fn get_products(&self) -> Vec<Product> {
        use crate::schema::products::dsl::*;
    
        self.run(move |conn| {
            products
                .load(conn)
                .unwrap() // diesel tries to get rid of any errors at compile-time
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
                .unwrap()
        }).await
    }

    // users
    /*
    pub async fn get_user(&self) -> User {
        use crate::schema::users::dsl::*;

        let user_data = self.run(move |conn| {
            users
                .select(id)
                .find(id)
                .load::<UserData>(conn)
                .unwrap()
        }).await;

        todo!()
    }
    */

    pub async fn create_user(&self, new_user: NewUser) -> User {
        use crate::schema::users;

        let user_data = self.run(move |conn| {
            diesel::insert_into(users::table)
                .values(&new_user)
                .get_result::<UserData>(conn)
                .unwrap()
        }).await;

        User(user_data.id)
    }
}
