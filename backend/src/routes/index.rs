use rocket::{post, http::{CookieJar, Status, Cookie}, serde::json::Json};

use crate::{database::Database, models::{NewUser, User}};

#[post("/register", data = "<new_user>")]
pub async fn register(db: Database, _cookies: &CookieJar<'_>, new_user: Json<NewUser>) -> Json<User> {
    let user = db.create_user(new_user.into_inner()).await;
    Json(user)
    //cookies.add_private(Cookie::new("user", serde_json::to_string(&user).unwrap()));
}

#[post("/login", data = "<new_user>")]
pub async fn login(db: Database, cookies: &CookieJar<'_>, new_user: Json<NewUser>) -> Status {
    match db.get_user(new_user.into_inner()).await {
        Some(user) => {
            cookies.add_private(Cookie::new("user", serde_json::to_string(&user).unwrap()));
            Status::Ok
        }
        None => Status::NotFound
    }
}

#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Status {
    cookies.remove_private(Cookie::named("user"));
    Status::Ok
}
