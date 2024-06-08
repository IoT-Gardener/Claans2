use diesel::prelude::*;
use rocket::FromForm;
use serde::{Deserialize, Serialize};

use super::schema::*;

#[derive(Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = orgs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Org {
    pub org_id: i32,
    pub org_name: String,
}

#[derive(Serialize, Deserialize, Insertable, FromForm)]
#[diesel(table_name = orgs)]
pub struct NewOrg {
    #[field(validate = len(1..))]
    pub org_name: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = claans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Claan {
    pub claan_id: i32,
    pub claan_name: String,
    pub org_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewClaan {
    pub claan_name: String,
    pub org_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub org_id: i32,
    pub claan_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub user_name: String,
    pub org_id: i32,
    pub claan_id: i32,
}
