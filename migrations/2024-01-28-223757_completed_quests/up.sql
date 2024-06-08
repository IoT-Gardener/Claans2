-- Your SQL goes here
CREATE TABLE IF NOT EXISTS completed_quests
(
    id              integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    data_completed  timestamp                                               NOT NULL DEFAULT CURRENT_DATE,
    earned_points   text                                                    NOT NULL,
    active_quest_id integer REFERENCES active_quests (id) ON DELETE CASCADE NOT NULL,
    user_id         integer REFERENCES users (id) ON DELETE CASCADE         NOT NULL,
    claan_id        integer REFERENCES claans (id) ON DELETE CASCADE        NOT NULL
);
CREATE INDEX idx_completed_quests_fkeys ON completed_quests (active_quest_id, user_id, claan_id);
