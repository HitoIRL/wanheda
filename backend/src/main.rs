#[macro_use] extern crate rocket;

#[get("/collection")]
fn collection() -> String {
    "{collection}".to_owned()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![collection])
}
