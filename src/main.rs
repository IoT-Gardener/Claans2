extern crate rocket;

use rocket::{launch, routes, get};
use rocket_dyn_templates::{context, Template};
pub mod models;
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
        .mount("/", routes![services::create_org_json])
        .mount("/", routes![services::create_org_form])
        .mount("/", routes![services::list_orgs])
        .attach(Template::fairing())
}
