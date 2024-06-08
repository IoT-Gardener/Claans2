use rocket::form::Form;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

use crate::models::*;
use crate::services;

#[get("/orgs")]
pub fn get_orgs() -> Template {
    let orgs = services::db::get_orgs();

    Template::render("pages/orgs", context! {orgs: &orgs})
}

#[post("/orgs", format = "multipart/form-data", data = "<org>")]
pub fn create_org(org: Form<NewOrg>) -> Redirect {
    // TODO Implement proper logging for failed writes
    // Only error handling currently is to render to user
    match services::db::create_org(org.into_inner()) {
        // TODO Rendering new org is wonky
        // reimplement with proper formatting for showing creation confirmation
        // maybe using {{#if new_org}} in the template and passing optionally?
        Ok(()) => Redirect::to(uri!(get_orgs)),
        Err(_) => panic!("Unable to write org"),
    }
}

// TODO Reimplement this function in API later
// Old json API style implementation
// #[post("/orgs", format = "json", data = "<org>")]
// pub fn create_org_json(org: Json<NewOrg>) -> Result<Created<Json<NewOrg>>> {
//     use self::schema::orgs::dsl::*;
//     let connection = &mut establish_connection_pg();
//
//     let new_org = NewOrg {
//         org_name: org.org_name.to_string(),
//     };
//
//     diesel::insert_into(orgs)
//         .values(&new_org)
//         .execute(connection)
//         .expect("Error creating new org.");
//     Ok(Created::new("/").body(org))
// }
