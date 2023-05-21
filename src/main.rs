#[macro_use] extern crate rocket;
use rocket::http::Status;
use rocket::response::{content, status};
use std::env;

#[get("/")]
fn index() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::ImATeapot, content::RawJson("{ \"hello\": \"world\" }"))
}

#[launch]
fn rocket() -> _ {
    env::set_var("ROCKET_ADDRESS", "0.0.0.0");
    if let Ok(port) = env::var("PORT") {
        env::set_var("ROCKET_PORT", port);
    }
    
    rocket::build().mount("/", routes![index])
}