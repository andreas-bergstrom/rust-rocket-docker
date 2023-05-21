#[macro_use] extern crate rocket;
use rocket::http::Status;
use rocket::response::{content, status};

#[get("/")]
fn index() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::ImATeapot, content::RawJson("{ \"hello\": \"world\" }"))
}

#[launch]
fn rocket() -> _ {
    env::set_var("ROCKET_ADDRESS", "0.0.0.0");
    rocket::build().mount("/", routes![index])
}