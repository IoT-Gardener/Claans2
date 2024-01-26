// @generated automatically by Diesel CLI.

diesel::table! {
    claans (claan_id) {
        claan_id -> Int4,
        claan_name -> Text,
        org_id -> Int4,
    }
}

diesel::table! {
    orgs (org_id) {
        org_id -> Int4,
        org_name -> Text,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        user_name -> Text,
        org_id -> Int4,
        claan_id -> Int4,
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
