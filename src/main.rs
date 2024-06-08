#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;

pub mod models;
mod routes;
pub mod schema;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::index::index])
        .mount("/", routes![routes::orgs::get_orgs])
        .mount("/", routes![routes::orgs::create_org])
        .attach(Template::fairing())
}
