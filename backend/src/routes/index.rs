use argon2::{password_hash::SaltString, Argon2, Algorithm, Version, Params, PasswordHasher, PasswordHash, PasswordVerifier};
use rand_core::OsRng;
use rocket::{post, http::{CookieJar, Status, Cookie}, serde::json::Json};

use crate::{database::Database, models::{NewUser, User}};

#[post("/register", data = "<new_user>")]
pub async fn register(db: Database, _cookies: &CookieJar<'_>, new_user: Json<NewUser>) -> Json<User> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2_params = Params::new(15728, 2, 1, None).unwrap();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::default(), argon2_params);
    let password_hash = argon2.hash_password(new_user.password.as_bytes(), &salt).unwrap().to_string();
    
    let new_user = NewUser {
        password: password_hash,
        ..new_user.into_inner()
    };

    let user = db.create_user(new_user).await;
    Json(user)
    //cookies.add_private(Cookie::new("user", serde_json::to_string(&user).unwrap()));
}

#[post("/login", data = "<new_user>")]
pub async fn login(db: Database, cookies: &CookieJar<'_>, new_user: Json<NewUser>) -> Status {
    let original_password = new_user.clone().into_inner().password;

    match db.get_user(new_user.into_inner()).await {
        Some(user) => {
            let parsed_hash = PasswordHash::new(&user.password).unwrap();
            if Argon2::default().verify_password(original_password.as_bytes(), &parsed_hash).is_ok() {
                cookies.add_private(Cookie::new("user", serde_json::to_string(&user).unwrap()));
                return Status::Ok;
            }

            Status::NotFound
        }
        None => Status::NotFound
    }
}

#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Status {
    cookies.remove_private(Cookie::named("user"));
    Status::Ok
}
