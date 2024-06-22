use rust_okapi_template::routes;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::todos::get_todos])
}
