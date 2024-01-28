// @generated automatically by Diesel CLI.

diesel::table! {
    claans (id) {
        id -> Int4,
        name -> Text,
        org_id -> Int4,
    }
}

diesel::table! {
    orgs (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    quest_definitions (id) {
        id -> Int4,
        description -> Nullable<Text>,
        reward -> Text,
        org_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        org_id -> Int4,
        claan_id -> Nullable<Int4>,
    }
}

diesel::joinable!(claans -> orgs (org_id));
diesel::joinable!(quest_definitions -> orgs (org_id));
diesel::joinable!(users -> claans (claan_id));
diesel::joinable!(users -> orgs (org_id));

diesel::allow_tables_to_appear_in_same_query!(claans, orgs, quest_definitions, users,);
