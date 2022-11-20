use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub price: f32,
    pub image: String,
}
