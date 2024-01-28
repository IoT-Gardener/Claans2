-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users
(
    id       integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name     text                                           NOT NULL,
    org_id   integer REFERENCES orgs (id) ON DELETE CASCADE NOT NULL,
    claan_id integer                                        REFERENCES claans (id) ON DELETE SET NULL
);
CREATE INDEX idx_users_fkeys ON users (claan_id, org_id);
