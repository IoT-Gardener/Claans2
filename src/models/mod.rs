use super::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = orgs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Org {
    pub org_id: i32,
    pub org_name: String,
}

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = claans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Claan {
    pub claan_id: i32,
    pub claan_name: String,
    pub org_id: i32,
}

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub org_id: i32,
    pub claan_id: i32,
}
