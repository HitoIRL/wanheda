use rocket::{request::{FromRequest, self}, Request, outcome::IntoOutcome};

#[derive(Debug)]
pub struct User(pub i32);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = i32;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(User)
            .or_forward(())
    }
}
