-- Your SQL goes here
CREATE TABLE IF NOT EXISTS quest_definitions
(
    id          integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    description text,
    reward      text                                           NOT NULL,
    org_id      integer REFERENCES orgs (id) ON DELETE CASCADE NOT NULL
);
CREATE INDEX idx_quest_definitions_fkeys ON quest_definitions (org_id);
