use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::response::Debug;

use crate::models::*;
use crate::schema;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_orgs() -> Vec<Org> {
    use schema::orgs::dsl::*;

    let connection = &mut establish_connection_pg();
    let results = orgs.load::<Org>(connection).expect("Error loading orgs");
    results
}

pub fn create_org(org: NewOrg) -> Result<()> {
    use self::schema::orgs::dsl::*;

    let connection = &mut establish_connection_pg();
    diesel::insert_into(orgs).values(&org).execute(connection)?;
    Ok(())
}
