// @generated automatically by Diesel CLI.

diesel::table! {
    claans (claan_id) {
        claan_id -> Int4,
        #[max_length = 255]
        claan_name -> Varchar,
        org_id -> Nullable<Int4>,
    }
}

diesel::table! {
    orgs (org_id) {
        org_id -> Int4,
        #[max_length = 255]
        org_name -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 255]
        user_name -> Varchar,
        org_id -> Nullable<Int4>,
        claan_id -> Nullable<Int4>,
    }
}

diesel::joinable!(claans -> orgs (org_id));
diesel::joinable!(users -> claans (claan_id));
diesel::joinable!(users -> orgs (org_id));

diesel::allow_tables_to_appear_in_same_query!(
    claans,
    orgs,
    users,
);
