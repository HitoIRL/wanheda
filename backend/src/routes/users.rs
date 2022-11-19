use rocket::{get, serde::json::Json};

use crate::{database::Database, models::User};

#[get("/users")]
pub async fn get_users(db: Database) -> Json<Vec<User>> {
    let users = db.get_users().await;
    Json(users)
}
