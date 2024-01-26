-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users
(
    user_id   INT GENERATED ALWAYS AS IDENTITY,
    user_name VARCHAR(255) NOT NULL,
    org_id    INT REFERENCES orgs (org_id) ON DELETE CASCADE,
    claan_id  INT REFERENCES claans (claan_id) ON DELETE CASCADE,
    PRIMARY KEY (user_id)
);
CREATE INDEX idx_users_fkeys ON users (claan_id, org_id);