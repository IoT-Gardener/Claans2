-- Your SQL goes here
CREATE TABLE IF NOT EXISTS claans
(
    id     integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name   text                                           NOT NULL,
    org_id integer REFERENCES orgs (id) ON DELETE CASCADE NOT NULL
);
CREATE INDEX idx_claans_fkeys ON claans(org_id);
