-- Your SQL goes here
CREATE TABLE IF NOT EXISTS active_quests
(
    id                  integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    description         text,
    reward              text                                                        NOT NULL,
    is_active           bool                                                        NOT NULL,
    quest_definition_id integer REFERENCES quest_definitions (id) ON DELETE CASCADE NOT NULL,
    org_id              integer REFERENCES orgs (id) ON DELETE CASCADE              NOT NULL
);
CREATE INDEX idx_active_quests_fkeys ON active_quests (org_id, quest_definition_id);
