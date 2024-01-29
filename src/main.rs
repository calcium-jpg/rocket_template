use rocket::{launch, routes};

use rocket_template::routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::index::get])
}
