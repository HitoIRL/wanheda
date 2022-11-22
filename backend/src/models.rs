use crate::schema::{products, users};

use diesel::prelude::*;
use rocket::{request::{self, FromRequest}, Request, outcome::IntoOutcome};
use serde::{Serialize, Deserialize};

// products
#[derive(Queryable, Serialize)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub price: f32,
    pub image: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = products)]
pub struct NewProduct { // TODO: rename
    pub title: String,
    pub price: f32,
    pub image: String,
}

// users
#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = i32;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request.cookies()
            .get_private("user")
            .and_then(|cookie| serde_json::from_str(cookie.value()).unwrap())
            .or_forward(())
    }
}

#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = users)]
pub struct NewUser { // TODO: rename
    pub username: String,
    pub password: String,
}
