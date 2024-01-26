-- Your SQL goes here
CREATE TABLE IF NOT EXISTS claans
(
    claan_id   integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    claan_name text NOT NULL,
    org_id     integer REFERENCES orgs (org_id) ON DELETE CASCADE NOT NULL
);
CREATE INDEX idx_claans_fkeys ON claans(org_id);
