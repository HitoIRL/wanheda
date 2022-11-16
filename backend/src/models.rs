use diesel::prelude::*;

#[derive(Queryable)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub price: f32,
}
