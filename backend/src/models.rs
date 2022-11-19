use crate::schema::products;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};

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
