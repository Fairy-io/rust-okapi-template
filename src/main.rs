use std::sync::Arc;

use rust_okapi_template::{db::connection::create_pool, routes};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let db_pool = create_pool();

    rocket::build()
        .manage(Arc::new(db_pool))
        .mount("/", routes![routes::todos::get_todos])
}
