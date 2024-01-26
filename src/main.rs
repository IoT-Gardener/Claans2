extern crate rocket;

use rocket::{launch, routes, get};
use rocket_dyn_templates::{context, Template};
pub mod models;
pub mod schema;
mod services;
use services::*;

#[get("/")]
fn index() -> Template {
    Template::render("pages/index", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![create_org_json])
        .mount("/", routes![create_org_form])
        .mount("/", routes![list_orgs])
        .attach(Template::fairing())
}
