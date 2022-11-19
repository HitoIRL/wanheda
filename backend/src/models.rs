use crate::schema::{products, users};

use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// products
#[derive(Queryable, Serialize)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub price: f32,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub title: String,
    pub price: f32,
}

// users
#[derive(Queryable)]
pub struct UserData {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}
