extern crate diesel;
extern crate rocket;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post};
use crate::models;
use crate::schema;
use rocket_dyn_templates::{context, Template};
use std::env;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::orgs)]
pub struct NewOrg {
    org_name: String
}

#[derive(Serialize, Deserialize)]
pub struct NewClaan {
    claan_name: String,
    org_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    user_name: String,
    org_id: i32,
    claan_id: i32,
}

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[post("/orgs", format = "json", data = "<org>")]
pub fn create_org(org: Json<NewOrg>) -> Result<Created<Json<NewOrg>>> {
    use self::schema::orgs::dsl::*;
    let connection = &mut establish_connection_pg();

    let new_org = NewOrg {
        org_name: org.org_name.to_string(),
    };

    diesel::insert_into(orgs)
        .values(&new_org)
        .execute(connection)
        .expect("Error creating new org.");
    Ok(Created::new("/").body(org))
}

#[get("/orgs")]
pub fn list_orgs() -> Template {
    use self::models::Org;
    let connection = &mut establish_connection_pg();
    let results = self::schema::orgs::dsl::orgs
        .load::<Org>(connection)
        .expect("Error loading orgs.");
    Template::render("orgs", context! {orgs: &results, count: results.len()})
}
