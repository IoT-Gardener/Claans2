// @generated automatically by Diesel CLI.

diesel::table! {
    active_quests (id) {
        id -> Int4,
        description -> Nullable<Text>,
        reward -> Text,
        is_active -> Bool,
        quest_definition_id -> Int4,
        org_id -> Int4,
    }
}

diesel::table! {
    claans (id) {
        id -> Int4,
        name -> Text,
        org_id -> Int4,
    }
}

diesel::table! {
    completed_quests (id) {
        id -> Int4,
        data_completed -> Timestamp,
        earned_points -> Text,
        active_quest_id -> Int4,
        user_id -> Int4,
        claan_id -> Int4,
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

diesel::joinable!(active_quests -> orgs (org_id));
diesel::joinable!(active_quests -> quest_definitions (quest_definition_id));
diesel::joinable!(claans -> orgs (org_id));
diesel::joinable!(completed_quests -> active_quests (active_quest_id));
diesel::joinable!(completed_quests -> claans (claan_id));
diesel::joinable!(completed_quests -> users (user_id));
diesel::joinable!(quest_definitions -> orgs (org_id));
diesel::joinable!(users -> claans (claan_id));
diesel::joinable!(users -> orgs (org_id));

diesel::allow_tables_to_appear_in_same_query!(
    active_quests,
    claans,
    completed_quests,
    orgs,
    quest_definitions,
    users,
);
