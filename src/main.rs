#[macro_use]
extern crate rocket;

use rocket::{get, launch, routes};
use rocket_dyn_templates::{context, Template};

pub mod models;
mod routes;
pub mod schema;
mod services;

#[get("/")]
fn index() -> Template {
    Template::render("pages/index", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![routes::orgs::get_orgs])
        .mount("/", routes![routes::orgs::create_org])
        .attach(Template::fairing())
}
