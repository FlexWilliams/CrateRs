#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, from CrateRs!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}