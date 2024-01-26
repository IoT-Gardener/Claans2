-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users
(
    user_id   integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_name text NOT NULL,
    org_id    integer REFERENCES orgs (org_id) ON DELETE CASCADE NOT NULL,
    claan_id  integer REFERENCES claans (claan_id) ON DELETE SET NULL
);
CREATE INDEX idx_users_fkeys ON users (claan_id, org_id);
