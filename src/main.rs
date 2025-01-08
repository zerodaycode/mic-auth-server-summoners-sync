mod api_rest;
mod domain;
mod infrastructure;

#[macro_use] extern crate rocket;
use canyon_sql::main;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
#[main]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/auth", routes![api_rest::controllers::auth::register])
}

